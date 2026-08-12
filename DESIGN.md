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
namespace firefly {
	function tick() {
		:timers[$timer] += 1
	}
}
```

```py
namespace firefly {
	function load() {
		board :timers
		:timers[$timer] = 0
	}
}
```

```py
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