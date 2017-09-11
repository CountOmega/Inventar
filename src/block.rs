 fn give_block_data(&self)
    {
        struct drops
        {
            drops:bool,
            drops_self:bool,
            drop_amount:i32,
            silk_touch:bool

        }

        struct stackable
        {
            is_stackable:bool,
            number:i32
        }

        struct id
        {
            id_name:str,
            hex_val:f32
        }

        impl id
        {
            //empty meanwhile
        }

        struct tool
        {
            tool_type:i32,
            tool_material:i32
        }

        struct meta_data
        {
            has_metadata:bool,
            metadata:String,
            blockstate[22]:arr
        }

        struct block_data
        {
            gravitation:bool,
            transparency:bool,
            bright:bool,
            piston:Vec, //
            expl_resistance:i8,
            hardness:f32,
            tool, 
            renewable:bool,
            inflammable:bool,
            drops,
            stackable,
            id,
            metadata
        }
        return block_data;
    }

}