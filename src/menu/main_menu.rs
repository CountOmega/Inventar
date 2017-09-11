
use glium;
use image;
use std::path::PathBuf;
use std;
use std::io::Cursor;
use glium::backend::glutin_backend::GlutinFacade;






pub fn draw()
{
   use glium::{DisplayBuild, Surface};
   let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        for ev in display.poll_events() 
        {
            match ev 
            {
                glium::glutin::Event::Closed => return, _ => ()
            }
        }
    }
    
}









