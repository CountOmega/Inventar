use glium;
use Item::Item;
use Inventar;



struct Slot
{
    slot_number:i32, 
    xdisplay:i32, 
    ydisplay:i32  

}

impl Slot

{
     fn zeichnen()
    {

    }
    /*
    fn test(Inventar i, i32 number, i32 x, i32 y)
    {
        i=inv;
        number=slot_number;
        x=x_display;
        y=y_display;
    }
    */
    fn give_anzahl(&self)
    {
        return self.number;
    }

    fn is_empty(&self)
    {
        if give_anzahl(&self)==0
        {
            return true;
        }
    }
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
            //convert inf f32 to hex
        }

        struct tool
        {
            tool_type:i32,
            tool_material:i32
        }

        struct meta_data
        {
            has_metadata:bool,
            metadata:str,
            blockstate[22]:arr
        }

        struct block_data
        {
            gravitation:bool,
            transparency:bool,
            bright:bool,
            piston:Vec,
            expl_resistance:i8,
            hardness:f32,
            tool, 
            renewable:bool,
            inflammable:bool,
            drops,
            id,
            metadata
        }
        return block_data;
    }

   

    fn inc(&self)
    {
        self.size = give_anzahl();
        self.size=self.size+1;
        i.add();
    }

    fn dec(&self)
    {
        self.size = give_anzahl();
        self.size=self.size-1;
        i.add();
    }

    fn insert(&self)
    {
        if is_empty()==false
           { i.add(item_number(i));}
        
    }
}