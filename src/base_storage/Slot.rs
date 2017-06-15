use glium;
use base_storage::Item::Item;
use base_storage::Inventar;



struct Slot
{
    slot_number:i32, 
    xdisplay:i32, 
    ydisplay:i32,  
    size:i32

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
    pub fn give_number(&self)->i32
    {
        return self.slot_number;
    }

    fn is_empty(&self)->bool
    {
        if give_number(&self)==0
        {
            return true;
        }
        else {return false;}
    }
   

    fn inc(&self)
    {
        self.size = give_number();
        self.size=self.size+1;
        //i.add();
    }

    fn dec(&self)
    {
        self.size = give_number();
        self.size=self.size-1;
        //i.add();
    }

    fn insert(&self)
    {
        if is_empty()==false
        { 
            i.add(item_number(i));
        }
        
    }
}