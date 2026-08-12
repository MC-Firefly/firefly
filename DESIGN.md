# Firefly

Firefly is a preprocessor for Minecraft Datapacks.

## Code examples

One file with two languages

```py
namespace firefly {
	function load() {
		# We can use Firefly's syntax...
		board :scores
		:scores[@a] = 0

		# And we can use MCF's.
		execute as @a run tellraw @s ["Your score was reset to ",{"score":{"name":"@s","objective":"firefly.scores"}}]
	}
}

# For some JSON files, Firefly has a custom syntax.
namespace minecraft {
	tag functions {
		load {
			firefly:load
		}
	}
}
```

Multiple files

```py
# tick.ff
namespace firefly {
	function tick() {
		:timers[$timer] += 1
	}
}
```

```py
# load.ff
namespace firefly {
	function load() {
		board :timers
		:timers[$timer] = 0
	}
}
```

```py
# tags.ff
namespace minecraft {
	tag functions {
		load {
			firefly:load
		}
		
		tick {
			firefly:tick
		}
	}
}
```                                                                                                     

Switch cases

```py
namespace firefly {
	function load() {
		board :temp
	}
	
	function case(arg) {
		:temp[sc] = $(arg)

		switch :temp[sc] {
			1 {
				function :first
			}
			2 {
				function :second
			}
			_ {
				function :other
			}
		}
	}
}
```

If/else

```py
namespace firefly {
	function elses() {
		as @a {
			if predicate firefly:sneaking {
				say Crouching
			} elif predicate firefly:flying {
				say Flying
			} else {
				say Not flying or crouchinhg
			}
		}
	}
}
```

Execute as, and at

```py
namespace firefly {
	function executes() {
		as @a at @s {
			if block ~ ~-0.2 ~ grass_block {
				say Touching grass
			}
		}
	}
}
```

Compile-time variables
```py
# Compile-time variables are special as they can be
# defined anywhere in a file, and used at any point
# after. Compile-time variables start with a * and
# can be used as such:
*PI_1000 = 3141

namespace firefly {
	function load() {
		# Note that as we exclude the : this scoreboard is just called consts.
		board consts
		consts[$pi] = *{PI_1000}
		tellraw @a {"score":{"name":"$pi","objective":"consts"}}
	}

	function enter_pi(arg) {
		# Again, just called temp
		board temp
		temp[$possibly_pi] = $(arg)

		if temp[$possibly_pi] matches *{PI_1000} {
			say You know what pi*1000 is!
		}
	}
}
```

Compile-time substitutions
```py
# You can use compile-time substitutions to reference
# functions, scoreboards and more in a more Firefly-like
# fashion, and without worrying about namespaces.
# Function substitutions are wrapped in the format of &{}.
# Scoreboard substitutions are wrapped in the format of &().

namespace firefly {
	function hello(name) {
		say Hello, $(name)!
	}
	function load() {
		board :myImportantBoard
		scoreboard players set joe &(:myImportantBoard) 1

		function &{:hello} {"name":"world"}
	}
}
```

MCFunction commands
```py
namespace firefly {
	function load() {
		# Let's say we define a board called weather...
		board weather

		# How do we use the normal Minecraft weather command now?
		# This will cause a syntax error...
		weather set clear
		
		# Just in case, you can preface your command with a /
		# No syntax error!
		/weather set clear

		# It can also be used to output whatever you'd like to the
		# generated MCF - like in Zoglin.
		/# This command will be in the final generated file.
		/and this is just invalid MCF, but will be inserted anyway.

		# Additionally, like Zoglin, you can preface multiple lines
		# with this syntax:

		/-
		weather set clear
		execute if score $sad weather matches 1 run weather set rain
		-/

		# However, you cannot use compile-time substitutions, as this
		# is inserted directly into the MCF.
		# Generally, Minecraft commands will not be overwritten by the
		# default Firefly keywords, so you shouldn't need to use this
		# syntax too much.
	}
}
```