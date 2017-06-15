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
   

    fn inc(&self)
    {
        self.size = give_anzahl();
        self.size=self.size+1;
        //i.add();
    }

    fn dec(&self)
    {
        self.size = give_anzahl();
        self.size=self.size-1;
        //i.add();
    }

    fn insert(&self)
    {
        if is_empty()==false
        { 
            //i.add(item_number(i));
        }
        
    }
}