## Code Edits
The code for each character’s moves can be found in their respective folder inside the main fighters folder. If a particular move’s script is not present, then it is still using the base game’s code and will need to be reimplemented if being changed

The dump for all vanilla acmd scripts can be found [here](https://github.com/WuBoytH/SSBU-Dumped-Scripts/tree/main/smashline-hdr).


## Romfs Edits

### Editing vl.prcxml
The vl.prcxml file contains parameters related to characters’ mechanics, articles, specials, and certain other moves. They can be found in `romfs/source/fighter/fightername/param`

A comprehensive list of each fighter’s variables can be found [here](https://docs.google.com/spreadsheets/d/1rwDPL1jhE-KuEbbcBkkTb44z1smJp9rYme_xb4BIJTk). Floats are shown as decimal or whole values whereas integers are displayed in hex (this should be converted to base-10 in HDR’s vl). Note that if there is a typo, a type mismatch, or some other issue, the game will not load the vl correctly and revert to vanilla params


### Editing fighter_param.prcxml
The fighter_param.prcxml file contains general physics parameters for all fighters. It can be found in `romfs/source/`. This file cannot be reloaded if changed unless the game is restarted

The full list of vanilla parameters can be found [here](https://docs.google.com/spreadsheets/d/1UdH2L0xi8dO6jyPCcO5e5-8j7SGzezxkeZuF1uP8uOw)


### Editing motion_patch.yaml
The motion_patch.yaml file contains a diff list of properties related to character and article animations. It can be found in `romfs/source/fighter/fightername/motion`

Vanilla dumps for all motion lists can be found [here](https://github.com/WuBoytH/SSBU-Dumped-Motion-Lists)


### Applying romfs changes
To apply the changes to an existing build, replace the respective file file found in `fighter/fightername/param` and restart the match. To have the changes export with the dev build, edit the version found in the `build` folder instead of `source`, but make sure to apply all changes to the source version once finished as that is what gets pushed to the repo


## Assets
To add new or adjusted animations, the config.json found in the root of hdr-assets will need to be updated. If the animation is replacing one that exists in vanilla, it only needs to be placed in the character’s c00 folder and added to the `share-to-add` list. If the animation is for an entirely new move, it needs to be added to all c0X folders and added to the `new-dir-files` list

For details on how to structure the assets folder for a pull request, please see the "Pull Request Components" section of the [Contributor Guide](https://github.com/HDR-Development/HewDraw-Remix/wiki/Contributor-Guide)


## Build Arguments
- `--release` - builds the entirety of HDR into a plugin
- `dev` - builds whatever fighters are specified into a reloadable development.nro. The rest of HDR is compiled into the usual plugin
- `only` - builds just the core engine, common, and the specified fighter(s) into a plugin
- `dev-only` - acts the same as `dev` but only outputs the development.nro


## Reloading with development.nro
While on the character select screen, pressing `L + R + Up Taunt` will reload dev.nro, allowing for easy iterative development

Exiting and re-entering a match will also reload the character's vl.prcxml and motion_patch.yaml. Reloading fighter_param or any fighters that use weapon_frame callbacks requires a game restart (the latter will crash upon reloading dev.nro and spawning articles that use said callback)