use chumsky::Parser;
use chumsky::prelude::*;
use std::fs::{create_dir_all, remove_dir_all, DirEntry, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};
use std::collections::HashSet;

#[derive(Clone, Debug)]
#[derive(PartialEq)]
enum Token<'src> {
    Comment(&'src str),
    Num(i32),
    Str(&'src str),
    Op(&'src str),
    Ctrl(char),
    Ident(&'src str),
    Resource(Option<&'src str>, &'src str),
    Selector(char),
    Board,
    Namespace,
    If,
    Unless,
    Else,
    Match,
}

enum Value {
    Num(f64),
    Score{name: String, objective: String},
    Binary(Box<(Value, BinaryOp, Value)>)
}

#[derive(Clone)]
enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
}

enum BoardType {
    Default,
    Fixed(i32),
}

enum Expr<'src> {
    Error,
    Value(Value),
    Local(&'src str),
    Board(&'src str, BoardType),
    Namespace(&'src str, Box<Vec<Self>>),
    Binary(Box<Vec<Self>>, BinaryOp, Box<Vec<Self>>),
    If(Box<Vec<Self>>, Box<Expr<'src>>, Box<Expr<'src>>),
}

fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<Token<'src>>> {
    let num = text::int(10)
        .to_slice()
        .from_str()
        .unwrapped()
        .map(Token::Num);

    let str_ = just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(Token::Str);

    let resource = text::ascii::ident().or_not()
        .then_ignore(just(':'))
        .then(text::ascii::ident())
        .map(|(namespace, path)| Token::Resource(namespace, path));

    let selector = just('@')
        .then(one_of("aenprs"))
        .map(|(_, c)| Token::Selector(c));

    let ident = text::ascii::ident().map(|ident: &str| match ident {
        "namespace" => Token::Namespace,
        "board" => Token::Board,
        "match" => Token::Match,
        "if" => Token::If,
        "unless" => Token::Unless,
        "else" => Token::Else,
        _ => Token::Ident(ident),
    });

    let op = one_of("+*-/=")
        .repeated()
        .at_least(1)
        .to_slice()
        .map(Token::Op);

    let ctrl = one_of("()[]{};,").map(Token::Ctrl);

    let comment = just("#")
        .then(any().and_is(just('\n').not()).repeated())
        .to_slice()
        .padded()
        .map(Token::Comment);

    let token = comment.or(resource).or(num).or(selector).or(str_).or(op).or(ctrl).or(ident);

    token
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}

pub fn build() -> anyhow::Result<()> {
    let source = Path::new("src/");
    if let Err(e) = remove_dir_all(Path::new("data")) {
        if e.kind() != io::ErrorKind::NotFound {
            return Err(anyhow::Error::from(e));
        }
    }
    visit_dirs(&source, &|entry: &DirEntry| {
        let path = entry.path();
        let relative = path
            .strip_prefix(Path::new(source))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        let mut file = File::open(&path)?;
        build_file(&mut file, relative)
    })
}

fn build_file(file: &mut File, _path: &Path) -> anyhow::Result<()> {
    let mut buf: String = String::new();
    file.read_to_string(&mut buf)?;

    let tokens = lexer()
        .parse(&buf)
        .into_result()
        .map_err(|errors| anyhow::Error::msg(format!("Failed to lex tokens: {errors:?}")))?;

    println!("{:?}", tokens);
    Ok(())
}

struct Context<'a> {
    namespace: Option<&'a str>,
    boards: HashSet<String>,
    file: Option<File>,
    file_path: Option<PathBuf>,
    function: Option<&'a str>,
}

impl Context<'_> {
    fn get_function(&mut self, name: &str) -> io::Result<&mut File> {
        let namespace = self.get_namespace();
        let path = Path::new("data")
            .join(namespace)
            .join("function").join(format!("{name}.mcfunction"));
        let option = Some(path.clone());
        if self.file.is_none() || self.file_path != option {
            let namespace = self.get_namespace();
            create_dir_all(format!("data/{}/function", namespace)).ok();
            self.file = Some(OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?);
            self.file_path = option;
        }

        Ok(self.file.as_mut().unwrap())
    }

    fn get_namespace(&self) -> &str {
        self.namespace.unwrap_or("minecraft")
    }
}

impl Default for Context<'_> {
    fn default() -> Self {
        Self { namespace: None, boards: HashSet::new(), file: None, file_path: None, function: None }
    }
}

fn write_line(context: &mut Context, line: impl AsRef<str>) -> anyhow::Result<()> {
    if let Some(function) = context.function {
        let function = context.get_function(function)?;
        function.write(line.as_ref().as_bytes())?;
        function.write(b"\n")?;
    } else {
        eprintln!("Cannot write outside a function");
    }
    //context.get_function()
    Ok(())
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry) -> anyhow::Result<()>) -> anyhow::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, &cb)?;
            } else {
                cb(&entry)?;
            }
        }
    }
    Ok(())
}