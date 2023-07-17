# Copyright
© 2023 Lilly & GeNT Developers archaic.archea@gmail.com  
License: [CNPL](https://git.pixie.town/thufie/npl-builder/raw/branch/main/cnpl.md)

# GeNT
This file covers the general architecture, and planned aspects of, GeNT

## Sections
GeNT is split into 3 types of sections, each section has specific use cases that allow for us to save on as much memory as possible.
The three sections are labeled the following
* Init          - Data/code only used during initialization, this memory will be released as soon as init mode is exitted
* Non-swappable  - Any data/code that is CRITICAL, and can NEVER be stored on disk while the OS is running, this code is guaranteed to always be in memory, sometimes for speed, sometimes because its required to access storage, as well as other reasons
* Swappable      - Data/Code that can be stored in non-volatile storage for later OS use, this includes some outer parts of the OS, as well as most tasks, although tasks can request that they are non-swappable
  
## Core Modules
GeNT is split up into several Core Modules, each of these serves a purpose in the OS.
* Init          - The Init core module is the same as the section specified in Sections, this is only used at the beginning, and is used for initialization. See the Init section of this document for more information
* Exit-Init     - Remove the Init components from memory
* Swapping      - A module dedicated to managing loading things into and out of RAM as needed to maintain free space, more information can be found in the Swap section.

## Init
The initialization process consists of starting up every Core on the system, setting up storage drivers, interrupt set up, among other system specific initialization processes.

## Swap
In some systems memory may not be very available, so we can use swapping to free up space in RAM, this can be applied on some parts of the kernel, as well as the majority of tasks. First when swapping out a page, we look for an available spot in swap storage, and once we find a valid block, we copy the page of RAM into it, and the in the PTE associated with the page, we store the ID of the spot in swap.