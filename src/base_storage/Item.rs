pub struct Item
{
    numeric_id:i32,


}

impl Item
{

    pub fn give_ID(&self)->i32
    {
        return self.numeric_id;
    }

    
}

    

struct item_data
    {
        id_name:String,
        hex_val:f32,
        durability:i32,
        stackable:bool,
            
    }