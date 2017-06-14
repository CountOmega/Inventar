use glium;
use Item.add();

mod Inventar;
mod Item;



struct Slot()
{
    size:i32; 
    inhalt:Vec; 
    slot_number:i32; 
    xdisplay:i32; 
    ydisplay:i32; 
    Inventar inv;
    Slot s;
    Item i;

     

}

impl Slot

{
    fn test(Inventar i, i32 number, i32 x, i32 y)
    {
        i=inv;
        number=slot_number;
        x=x_display;
        y=y_display;
    }

    fn give_anzahl(Slot s)
    {
        return give_anzahl(s);
    }

    fn is_empty(Slot s)
    {
        if give_anzahl(s)==0;
        return true;
    }
    fn give_meta_data(Slot s)
    {
        //return anzahl, block_id, ...;
    }

    fn zeichnen()
    {

    }

    fn inc(Slot s)
    {
        size = give_anzahl(s);
        size=size+1;
        item.add();
    }

    fn dec(Slot s)
    {
        size = give_anzahl(s);
        size=size-1;
        item.add();
    }

    fn insert(Slot s)
    {
        if (is_empty(s)==false)
            item.add(item_number(i));
    }