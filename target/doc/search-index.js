var searchIndex = {};
searchIndex["celly"] = {"doc":"Library for building cellular automata.","items":[[0,"grid","celly","Module contains implemented grids and neighorhoods.",null,null],[3,"EmptyState","celly::grid","Dummy evolution state to be used with\ncellular automata where this concept is\nnot applicable.",null,null],[0,"twodim","","2D grid with neighbors iter and custom internal coordinate.",null,null],[3,"GridCoord","celly::grid::twodim","Coordinate for TwodimGrid. Need for custom coordinate\nraises from the fact that grid is using one-dimensional\n`Vec` to store cells. This coordinate can be constructed\nfrom offset and grid size.",null,null],[3,"TwodimGrid","","2D grid. Implemented with two buffers.\nThey are swapped on every evolution step.\nOld buffer is used for read-only neighbors data.\nNew buffer is writable and mutated through update process.\nGrid uses one-dimensional `Vec` to store cells.",null,null],[11,"clone","","",0,null],[11,"eq","","",0,null],[11,"ne","","",0,null],[11,"fmt","","",0,null],[11,"from_offset","","Constructs GridCoord from given offset (in one-dimensional\narray or `Vec`) and grid size.",0,{"inputs":[{"name":"i32"},{"name":"i32"},{"name":"i32"}],"output":{"name":"gridcoord"}}],[11,"from_2d","","",0,{"inputs":[{"name":"i32"},{"name":"i32"}],"output":{"name":"self"}}],[11,"x","","",0,null],[11,"y","","",0,null],[11,"new","","Constructs TwodimGrid with given ROWSxCOLS, neighborhood\nstrategy and initial evolution state.",1,{"inputs":[{"name":"i32"},{"name":"i32"},{"name":"n"},{"name":"state"}],"output":{"name":"self"}}],[11,"update","","",1,null],[11,"set_cells","","",1,null],[11,"cells","","",1,null],[11,"state","","",1,null],[11,"dimensions","","",1,null],[0,"nhood","celly::grid","Module contains several implemented neighborhoods.",null,null],[3,"MooreNhood","celly::grid::nhood","Implements Moore neighborhood.\n0 | 1 | 2\n3 | x | 4\n5 | 6 | 7\nx - given coord. Neighbors is numbered in order they returned.",null,null],[3,"VonNeumannNhood","","Implements Von Neumann neighborhood.\n- | 0 | -\n1 | x | 2\n- | 3 | -\nx - given coord. Neighbors is number in order they returned.",null,null],[11,"new","","Just constructor.",2,{"inputs":[],"output":{"name":"self"}}],[11,"neighbors","","",2,null],[11,"neighbors_count","","",2,null],[11,"new","","Just constructor.",3,{"inputs":[],"output":{"name":"self"}}],[11,"neighbors","","",3,null],[11,"neighbors_count","","",3,null],[11,"update","celly::grid","",4,null],[0,"engine","celly","Module contains different engines which\ncan be used to run evolutions.",null,null],[0,"sequential","celly::engine","Most simple engine that runs evolution\nsequentially. Useful for test purposes\nand with grids which implemented interior parallelism.",null,null],[3,"Sequential","celly::engine::sequential","Engine generic over Cell and Consumer running\nevolution sequentially.",null,null],[11,"new","","Constructs engine with given Grid and Consumer.",5,{"inputs":[{"name":"g"},{"name":"con"}],"output":{"name":"self"}}],[11,"run_times","","",5,null],[0,"traits","celly","Interfaces on which this library is built.",null,null],[8,"EvolutionState","celly::traits","Trait represents global state of the\nentire simulation which can be updated\nindependently from any particular cell.\nConsider to store there as much as you can\nbecause this data will be created only once\nand will not be copied.",null,null],[10,"update","","Method is called once between cells&#39; updates.",6,null],[8,"Cell","","Main trait should be implemented in user&#39;s code.\nSuch structs contain main logic of cellular\nautomaton. Grids can handle only one cell,\nso it should be all-in-one.",null,null],[16,"Coord","","Coords supported by Cell.",7,null],[16,"State","","Global state of evolution.",7,null],[10,"update","","This method is called for every instance of Cell\nin grid. Cell can mutate itself. Grid should pass\nneighbors, previous version of this Cell and the \nglobal state.",7,null],[10,"with_coord","","Constructs Cell with given coord.",7,{"inputs":[{"name":"c"}],"output":{"name":"self"}}],[10,"coord","","Getter for cell&#39;s coordinate.",7,null],[10,"set_coord","","Setter for cell&#39;s coordinate.",7,null],[8,"Nhood","","Represents neighborhood for automata.",null,null],[16,"Coord","","Coords this nhood supports.",8,null],[10,"neighbors","","Method returns for any given coord\ncoordinates of surrounding neighbors.",8,null],[10,"neighbors_count","","Hint for grid.",8,null],[8,"Coord","","Basic coordinate with three components.\nThis crate is used for creating automata\non 2D grids for now, so `z` has default impl.",null,null],[10,"from_2d","","Build coord from any other representation of coord.",9,{"inputs":[{"name":"i32"},{"name":"i32"}],"output":{"name":"self"}}],[10,"x","","Returns `x` component.",9,null],[10,"y","","Returns `y` component.",9,null],[11,"z","","Returns `z` component.",9,null],[8,"Grid","","Grid stores cells and updates them. Also\ngrid contains global evolution state.",null,null],[16,"Cell","","Grid wants to work with them.",10,null],[16,"Coord","","Grid knows how to work with them.",10,null],[10,"update","","One step in evolution.",10,null],[10,"state","","Getter for evolution state.",10,null],[10,"cells","","Getter for all cells. It is `Vec` because\nRust does not have abstract return types for now.\nSo custom grids are doomed to use `Vec`s internally.",10,null],[10,"dimensions","","Returns `Coord` with rows and cols counts of grid (2D).\n3D grids would have more dimensions.",10,null],[10,"set_cells","","This method gives an ability to change grid externally.\nIt could be done from consumer, for example\n(consider an app where you reacting to user input),\nor from engine (consider distributed engine received\nupdates from nodes).",10,null],[8,"Consumer","","Interface to the outer world.",null,null],[16,"Cell","","Cells supported by this consumer. Helps\nto use cells from grid directly if grid\nhas the same cell in it.",11,null],[10,"consume","","Called once when all cells has been updated.",11,null],[8,"Engine","","Interlayer between grid and consumer(s).",null,null],[10,"run_times","","Runs evolution fixed number of times.",12,null]],"paths":[[3,"GridCoord"],[3,"TwodimGrid"],[3,"MooreNhood"],[3,"VonNeumannNhood"],[3,"EmptyState"],[3,"Sequential"],[8,"EvolutionState"],[8,"Cell"],[8,"Nhood"],[8,"Coord"],[8,"Grid"],[8,"Consumer"],[8,"Engine"]]};
initSearch(searchIndex);
