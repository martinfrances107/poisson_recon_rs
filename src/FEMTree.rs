use std::marker::PhantomData;

enum Flag {
    //SpaceFlag = 1u8<<0;
    //todo add more.
}

// search for clas FEMTree
#[derive(Debug, Default)]
#[allow(nonstandard_style)]
struct FEMTree<const DIM: usize, REAL> {
    pd: PhantomData<REAL>,
    node_allocators: Vec<u8>,
    local_depth: i32,
    local_offset: i32,
    node_count: usize,
}

trait NodeInitializerTrait {
    fn neighbor_key() {}
}

struct NodeInitializer {}

impl<const D: usize, REAL> FEMTree<D, REAL>
where
    REAL: Default,
{
    fn new(block_size: Option<usize>) -> Self {
        let node_allocators = vec![];
        if let Some(_block_size) = block_size {
            // ThreadPool
        }

        let mut s = Self {
            node_count: 0,
            node_allocators,
            ..Default::default()
        };

        // let tree = Self::node_initializer();
        s.init_children();

        s
    }

    fn init_children(&mut self) {
        todo!()
    }

    const fn node_count(&self) -> usize {
        self.node_count
    }
}

// impl FEMTreeNodeData{
// // void setGhostFlag( bool f ) const { if( f ) flags |= GHOST_FLAG ; else flags &= ~GHOST_FLAG; }
// 		// bool getGhostFlag( void ) const { return ( flags & GHOST_FLAG )!=0; }
// 		// void setDirichletNodeFlag( bool f ) const { if( f ) flags |= DIRICHLET_NODE_FLAG ; else flags &= ~DIRICHLET_NODE_FLAG; }
// 		// bool getDirichletNodeFlag( void ) const { return ( flags & DIRICHLET_NODE_FLAG )!=0; }
// 		// void setDirichletElementFlag( bool f ) const { if( f ) flags |= DIRICHLET_ELEMENT_FLAG ; else flags &= ~DIRICHLET_ELEMENT_FLAG; }
// 		// bool getDirichletElementFlag( void ) const { return ( flags & DIRICHLET_ELEMENT_FLAG )!=0; }
// 		// void setScratchFlag( bool f ) const { if( f ) flags |= SCRATCH_FLAG ; else flags &= (unsigned char)~SCRATCH_FLAG; }
// 		// bool getScratchFlag( void ) const { return ( flags & SCRATCH_FLAG )!=0; }
// }
