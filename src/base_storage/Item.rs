
pub struct Item
{
    id:i32
}

impl Item
{
    pub fn add()
    {}

    fn remove()
    {}

    pub fn item_number()
    {
        
    }

    pub fn give_ID(&self)
    {
        return self.id;
    }

    pub fn give_item_data(&self)
    {
        struct id
        {
            id_name:str,
            hex_val:f32
        }

        struct item_data
        {
            durability:i32,
            stackable:bool,
            id
        }
        return item_data;
    }
}