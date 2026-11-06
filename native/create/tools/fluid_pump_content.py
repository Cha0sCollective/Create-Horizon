#!/usr/bin/env python3
"""Generate the selected, straight-only FLUIDS-005 pump/pipe content.

The block IDs and state intent follow the pinned Create references. Geometry is
authored for this private port with vanilla texture IDs; no original art bytes
or full upstream resource pack are copied into the repository.

Call generate(CONTENT) from the shared content generator after composition.
This module writes only new pump/pipe-specific content paths.
"""

import json
from pathlib import Path


def generate(content: Path) -> None:
    def write(path: str, value: dict) -> None:
        target = content / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(json.dumps(value, indent=2) + "\n", encoding="utf-8")

    directions = ["down", "up", "north", "south", "west", "east"]
    properties = {
        "hardness": 1.5,
        "explosion_resistance": 6,
        "requires_tool": True,
        "render_shape": "MODEL",
        "sounds": "minecraft:andesite",
    }
    write("data/create/horizon/block/mechanical_pump.json", {
        "template": "minecraft:glass",
        "item": True,
        "properties": properties,
        "state_properties": {"facing": {"type": "enum", "values": directions}},
        "default_state": {"facing": "east"},
        "shapes": {"default": [[0, 0, 0, 1, 1, 1]]},
        "collision_shapes": {"default": [[0, 0, 0, 1, 1, 1]]},
    })
    pipe_shapes = {
        "x": [[0, .25, .25, 1, .75, .75]],
        "y": [[.25, 0, .25, .75, 1, .75]],
        "z": [[.25, .25, 0, .75, .75, 1]],
    }
    write("data/create/horizon/block/fluid_pipe.json", {
        "template": "minecraft:glass",
        "item": True,
        "properties": {**properties, "sounds": "minecraft:copper_block"},
        "state_properties": {"axis": {"type": "enum", "values": ["x", "y", "z"]}},
        "default_state": {"axis": "x"},
        "shapes": {"property": "axis", "values": pipe_shapes},
        "collision_shapes": {"property": "axis", "values": pipe_shapes},
    })

    # The pump model's north face is its fluid front. Rotations follow FACING.
    faces = {
        "down": {"texture": "#body"},
        "up": {"texture": "#body"},
        "north": {"texture": "#front"},
        "south": {"texture": "#back"},
        "west": {"texture": "#body"},
        "east": {"texture": "#body"},
    }
    write("assets/create/models/block/mechanical_pump/block.json", {
        "textures": {
            "body": "minecraft:block/andesite",
            "front": "minecraft:block/copper_block",
            "back": "minecraft:block/iron_block",
            "particle": "minecraft:block/andesite",
        },
        "elements": [{"from": [0, 0, 0], "to": [16, 16, 16], "faces": faces}],
    })
    rotations = {
        "north": {},
        "south": {"y": 180},
        "west": {"y": 270},
        "east": {"y": 90},
        "up": {"x": 270},
        "down": {"x": 90},
    }
    write("assets/create/blockstates/mechanical_pump.json", {
        "variants": {
            "facing=" + face: {"model": "create:block/mechanical_pump/block", **rotation}
            for face, rotation in rotations.items()
        }
    })

    pipe_faces = {face: {"texture": "#all"} for face in
                  ["down", "up", "north", "south", "west", "east"]}
    write("assets/create/models/block/fluid_pipe/straight.json", {
        "textures": {
            "all": "minecraft:block/copper_block",
            "particle": "minecraft:block/copper_block",
        },
        "elements": [
            {"from": [4, 4, 0], "to": [12, 12, 16], "faces": pipe_faces},
            {"from": [3, 3, 0], "to": [13, 13, 2], "faces": pipe_faces},
            {"from": [3, 3, 14], "to": [13, 13, 16], "faces": pipe_faces},
        ],
    })
    write("assets/create/blockstates/fluid_pipe.json", {
        "variants": {
            "axis=x": {"model": "create:block/fluid_pipe/straight", "y": 90},
            "axis=y": {"model": "create:block/fluid_pipe/straight", "x": 90},
            "axis=z": {"model": "create:block/fluid_pipe/straight"},
        }
    })
    for name, parent in [
        ("mechanical_pump", "mechanical_pump/block"),
        ("fluid_pipe", "fluid_pipe/straight"),
    ]:
        write("assets/create/models/item/" + name + ".json", {
            "parent": "create:block/" + parent,
        })
        write("assets/create/items/" + name + ".json", {
            "model": {"type": "minecraft:model", "model": "create:item/" + name},
        })
        write("data/create/loot_table/blocks/" + name + ".json", {
            "type": "minecraft:block",
            "pools": [{"rolls": 1, "entries": [{
                "type": "minecraft:item", "name": "create:" + name,
            }]}],
        })


if __name__ == "__main__":
    generate(Path(__file__).resolve().parents[1] / "content")
