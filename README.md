# Scardoc Generator

Unofficial ScarDoc generator, made by the Battlegrounds Development Team.

## Usage
To use the application simply run it through Powershell/Cmd with a path to the scar directory you wish to generate a scardoc file for. For example:

```bash
./coh3-scardoc-gen "E:\coh3-dev\coh3-scar\scar"
```

This will generate a `scardoc.json` file that you can use to get an easy and searchable documentation of your scar repository.

You may also pass `-g` to generate the scardoc of the specified scar repository.

It's also possible to merge different scardoc files. This is useful if you need to update a manually defined scardoc with an updated scardoc. For instance you can merge `scardoc_manual.json` into `scardoc_generated.json` by doing:

```bash
./coh3-scardoc-gen -m scardoc_generated.json scardoc_manual.json
```

The `-m` command will accept any amount of scardoc files and merge them together into one scardoc file.

### Documenting Scar Code

The generator currently picks up functions to be documented if they're in the format
```lua
--? @shortdesc Converts a 2D top down position to a 3D ScarPosition. returns Position, if y-height is nil, y-height = ground height, terrain ground or walkable
--? @extdesc
--? 3D ScarPositions have the x axis left to right, the z axis in to out, and the y axis down to up (y axis represents the height of the terrain).  Use this function to convert a top-down 2D position to a 3D world position.\n\n
--? Note: (0,0) is in the center of the map.
--? @result Position
--? @args Real xpos, Real zpos, Real ypos
function Util_ScarPos(xpos, zpos, ypos)
	if ypos == nil then
		ypos = World_GetHeightAt(xpos,zpos)
	end
	return World_Pos(xpos, ypos, zpos)
end
```

Meaning lines starting with `--?` are considred scardoc material and will be considered when generating the documentation for a function. The documented function must immediately follow the scardoc comments for the generator to associate them.

We currently support the following scardoc directives:
| Name         | Description     |
|:-------------|:----------------|
| @extdesc     | The extended description of the scar function - a more detailed description of how the function works and what each argument specifically does. This directive may expand over multiple scardoc comment lines |
| @shortdesc   | The short and simple description of the function |
| @result      | The type returned by the function |
| @args        | The type of the arguments and may be marked optional. Arguments can be marked optional by putting them inside a pair of square brackets `[]` |

## Build

The project is `Cargo` compliant and can be built using
```bash
cargo build
```

## Test

The project is testable and all tests can be run with
```bash
cargo test
```

## TODO

Features we'd like to implement at some point

* Additional directives (@example, @argdesc, @group)
* Be able to specify output file destination
* Extract default values (Will require a proper Lua parser)
* Global constant values
* List of types
