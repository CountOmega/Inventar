use glium;
use Item.add();

mod Inventar;
mod Item;
//


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
/**



    /**
     * Check if the stack is allowed to be placed in this slot, used for armor slots as well as furnace fuel.
     */
    public boolean isItemValid(ItemStack stack)
    {
        return true;
    }

    /**
     * Helper fnct to get the stack in the slot.
     */
    public ItemStack getStack()
    {
        return this.inventory.getStackInSlot(this.slotIndex);
    }

    /**
     * Returns if this slot contains a stack.
     */
    public boolean getHasStack()
    {
        return !this.getStack().func_190926_b();
    }

    /**
     * Helper method to put a stack in the slot.
     */
    public void putStack(ItemStack stack)
    {
        this.inventory.setInventorySlotContents(this.slotIndex, stack);
        this.onSlotChanged();
    }

    /**
     * Called when the stack in a Slot changes
     */
    public void onSlotChanged()
    {
        this.inventory.markDirty();
    }

    /**
     * Returns the maximum stack size for a given slot (usually the same as getInventoryStackLimit(), but 1 in the case
     * of armor slots)
     */
    public int getSlotStackLimit()
    {
        return this.inventory.getInventoryStackLimit();
    }

    public int getItemStackLimit(ItemStack stack)
    {
        return this.getSlotStackLimit();
    }

    @Nullable
    public String getSlotTexture()
    {
        return null;
    }

    /**
     * Decrease the size of the stack in slot (first int arg) by the amount of the second int arg. Returns the new
     * stack.
     */
    public ItemStack decrStackSize(int amount)
    {
        return this.inventory.decrStackSize(this.slotIndex, amount);
    }

    /**
     * returns true if the slot exists in the given inventory and location
     */
    public boolean isHere(IInventory inv, int slotIn)
    {
        return inv == this.inventory && slotIn == this.slotIndex;
    }

    /**
     * Return whether this slot's stack can be taken from this slot.
     */
    public boolean canTakeStack(EntityPlayer playerIn)
    {
        return true;
    }

    /**
     * Actualy only call when we want to render the white square effect over the slots. Return always True, except for
     * the armor slot of the Donkey/Mule (we can't interact with the Undead and Skeleton horses)
     */
    public boolean canBeHovered()
    {
        return true;
    }
}




*/