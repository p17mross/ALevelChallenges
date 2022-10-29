use std::collections::VecDeque;
use std::fmt::Debug;
use std::mem::take;

use glium::glutin::event::StartCause;
use glium::glutin::event_loop::ControlFlow;
use glium::glutin::event::ElementState;
use glium::{glutin, SwapBuffersError};
use glium::glutin::dpi::{PhysicalSize, LogicalSize, PhysicalPosition, LogicalPosition};
use glium::backend::glutin::DisplayCreationError;
use glutin::CreationError;
use glutin::event::Event;

use crate::{Scene, Frame};

///Represents the resolution of a window
#[derive(Debug)]
pub enum Resolution {
    Fullscreen,
    Physical (u32, u32),
    Logical (f64, f64)
}

#[derive(Debug)]
pub enum Position {
    Physical (i32, i32),
    Logical (f32, f32), 
}

///Trait with callbacks for a window
pub trait WindowCallback : Debug{
    fn on_error(&mut self, window: &mut Window, _error: WindowRuntimeError){println!("Closing window");window.close();}
    fn on_close(&mut self, window: &mut Window){window.close();}

    fn on_resize(&mut self, _window: &mut Window, _resolution: Resolution){}
    fn on_move(&mut self, _window: &mut Window, _position: Position){}

    fn on_tick(&mut self, _window: &mut Window, _frame: &Frame){}
}

#[derive(Debug)]
pub struct WindowCallbackDefault;
impl WindowCallback for WindowCallbackDefault {}

/// Different actions that can be performed on a window
/// Used to communicate from functions such as Window::close and Window::set_title to mainloop
#[derive(Debug)]
enum WindowAction {
    Close,
    SetTitle(String),

    UpdatePosition,
    UpdateResolution,
}

pub type ScanCode = glium::glutin::event::ScanCode;
pub type KeyCode = glium::glutin::event::VirtualKeyCode;



/// Struct representing a window
#[derive(Debug)]
pub struct Window
{
    callbacks: Option<Box<dyn WindowCallback>>,
    pub target_framerate: u64,
    scene: Option<Scene>,

    actions: VecDeque<WindowAction>,
    pub(crate) frame: Frame,

    event_loop: Option<glutin::event_loop::EventLoop<()>>,
    event_loop_started: bool,
    pub(crate) display: glium::Display,
}

///Represents an error that can occur with a window
#[derive(Debug)]
pub enum WindowCreationError {
    OsError(String),
    NoSupportedBackend,
    PlatformSpecific(String),
    Multiple(Vec<WindowCreationError>)
}

impl WindowCreationError {
    ///Creates a WindowCreationError from a DisplayCreationError
    fn from(display_error: DisplayCreationError) -> WindowCreationError {
        match display_error {
            DisplayCreationError::GlutinCreationError(e) => WindowCreationError::from_glutin(e),
            DisplayCreationError::IncompatibleOpenGl (_) => WindowCreationError::NoSupportedBackend
        }
    }
    ///Creates a WindowCreationError from a CreationError
    fn from_glutin(glutin_error: CreationError) -> WindowCreationError{
        match glutin_error {
            CreationError::OsError(s) => WindowCreationError::OsError(s),
            CreationError::NotSupported(_)
            | CreationError::NoBackendAvailable(_)
            | CreationError::RobustnessNotSupported
            | CreationError::OpenGlVersionNotSupported
            | CreationError::NoAvailablePixelFormat => WindowCreationError::NoSupportedBackend,
            CreationError::PlatformSpecific(s) => WindowCreationError::PlatformSpecific(s),
            CreationError::Window(_) => WindowCreationError::OsError("Error Creating Window".to_string()),
            CreationError::CreationErrors(cerrors) => {
                    let mut werrors: Vec<WindowCreationError> = Vec::with_capacity(cerrors.len());
                    for e in cerrors {
                        werrors.push(WindowCreationError::from_glutin(*e));
                    }
                    WindowCreationError::Multiple(werrors)
                }
        }
    }
}

#[derive(Debug)]
pub enum WindowIconChangeError {
    Io(std::io::Error),
    ImageError,
    BadIcon,
    Os(std::io::Error)
}
pub enum WindowRuntimeError {
    ContextLost,
}

impl Window {
    ///Constructs a new window
    pub fn new (callbacks: Option<Box<dyn WindowCallback>>, resolution: Resolution, title: String) -> Result<Self, WindowCreationError> {
        let event_loop = glutin::event_loop::EventLoop::new();
        let mut window_builder = glutin::window::WindowBuilder::new().with_title(title);
        window_builder = match resolution {
            Resolution::Fullscreen => window_builder.with_fullscreen(Some(glutin::window::Fullscreen::Borderless(event_loop.available_monitors().next()))),
            Resolution::Physical (width, height) => window_builder.with_inner_size(glutin::dpi::Size::Physical(PhysicalSize{width:width, height:height})),
            Resolution::Logical (width, height) => window_builder.with_inner_size(glutin::dpi::Size::Logical(LogicalSize{width:width, height:height}))
        };
        let context_builder = glutin::ContextBuilder::new().with_depth_buffer(24);
        let display = match glium::Display::new(window_builder, context_builder, &event_loop) {
            Ok (d) => d,
            Err (e) => return Err(WindowCreationError::from(e))
        };
        
        let frame = Frame {
            display: crate::Display {
                resolution: display.get_framebuffer_dimensions(),
                position: match display.gl_window().window().inner_position() {
                    Ok(pp) => (pp.x.clone(), pp.y.clone()),
                    Err(_) => (0, 0)
                },
            },
            time: Default::default(),
            input: Default::default(),
        };

        Ok (
            Window {
                callbacks: Some(callbacks.unwrap_or(Box::new(WindowCallbackDefault{}))),
                target_framerate: 60,
                scene: None,

                actions: VecDeque::new(),
                frame: frame,

                event_loop: Some(event_loop), 
                event_loop_started: false,
                display: display,
            }
        )
    }

    ///Closes the window
    pub fn close (&mut self) {
        self.actions.push_back(WindowAction::Close);
    }

    ///Sets the title of a window
    pub fn set_title (&mut self, title: String) {
        self.actions.push_back(WindowAction::SetTitle(title));
    }

    ///Gets the scene the window is running
    pub fn get_scene (&self) -> &Option<Scene> {
        &self.scene
    }

    pub fn set_icon (&mut self, path: String) -> Result<(), WindowIconChangeError> {
        let icon_image = match image::open(path) {
            Ok(i) => i,
            Err(e) => match e {
                image::ImageError::IoError(e) => return Err(WindowIconChangeError::Io(e)),
                _ => return Err(WindowIconChangeError::ImageError)
            }
        };

        let icon_r = glutin::window::Icon::from_rgba(
            icon_image.as_rgba8().unwrap().clone().into_vec(),
            icon_image.width(),
            icon_image.height()
        );
        let icon;

        match icon_r {
            Ok(i) => {icon = i;},
            Err(e) => match e {
                glutin::window::BadIcon::OsError(e) => return Err(WindowIconChangeError::Os(e)),
                _ => return Err(WindowIconChangeError::BadIcon)
            }
        }

        self.display.gl_window().window().set_window_icon(Some(icon));

        Ok(())
    }

    ///Sets the scene the window is running
    pub fn set_scene (&mut self, mut new_scene: Scene) {
        if let Some(old_scene) = take(&mut self.scene) {
            self.scene = None;
            old_scene.remove(self);
        }

        new_scene.insert(self);

        self.scene = Some(new_scene);
    }

    pub fn set_resolution(&mut self, resolution: &Resolution) {
        match resolution {
            Resolution::Physical(width, height) => {
                self.display.gl_window().window().set_fullscreen(None);
                self.display.gl_window().window().set_inner_size(PhysicalSize { width: *width, height: *height });
            },
            Resolution::Logical(width, height) => {
                self.display.gl_window().window().set_fullscreen(None);
                self.display.gl_window().window().set_inner_size(LogicalSize { width: *width, height: *height });
            },
            Resolution::Fullscreen => {
                self.display.gl_window().window().set_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));
                //self.ignore_size_change = true;
            }
        }
        self.actions.push_back(WindowAction::UpdateResolution);
    }

    pub fn set_position(&mut self, position: &Position) {
        //println!("{:?}", position);
        match position {
            Position::Physical(x, y) => {
                self.display.gl_window().window().set_outer_position(PhysicalPosition::new(*x, *y))
            },
            Position::Logical(x, y) => {
                self.display.gl_window().window().set_outer_position(LogicalPosition::new(*x, *y))
            }
        }
        self.actions.push_back(WindowAction::UpdatePosition);
    }

    fn run_actions(&mut self) -> Result<(), ()> {
        for action in self.actions.iter() {
            match action {
                //TODO: call scene's quit method first
                WindowAction::Close => {return Err(())},
                WindowAction::SetTitle(title) => self.display.gl_window().window().set_title(&title[..]),

                WindowAction::UpdatePosition => self.frame.display.position = match self.display.gl_window().window().inner_position() {
                    Ok(pp) => (pp.x, pp.y),
                    Err(_) => (0, 0)
                },
                WindowAction::UpdateResolution => {
                    let ps = self.display.gl_window().window().inner_size();
                    self.frame.display.resolution = (ps.width, ps.height);
                }
            }
        }
        self.actions.clear();
        return Ok(());
    }

    ///Called every frame, is responsible for calling logic and rendering code
    fn tick(&mut self, control_flow: &mut ControlFlow) {

        let frame_time = std::time::Instant::now();

        if self.event_loop_started {
            self.frame.time.delta_time = frame_time - self.frame.time.frame_time;      
            self.frame.time.frames += 1;
        }
        else {
            self.frame.time.delta_time = std::time::Duration::from_secs(0);
            self.frame.time.frames = 0;
        }

        self.frame.time.frame_time = frame_time;

        //Set window to wait for next frame
        let next_frame_time = frame_time +
            std::time::Duration::from_nanos(1_000_000_000 / self.target_framerate);
       *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        if let Some(mut callbacks) = take(&mut self.callbacks) {
            callbacks.on_tick(self, &self.frame.clone());

            //Handle actions queued by on_tick() call
            //Prevents another run of the event loop from happening if close() was called during the callback
            //Also means other properties such as window title are updated on the correct frame 
            
            match self.run_actions() {
                Ok(()) => (),
                Err(()) => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
            }

            self.callbacks = Some(callbacks);
        }
        
        if let Some(mut scene) = take(&mut self.scene) {

            scene.tick(&self);

            let mut target = self.display.draw();
            
            scene.render(&mut target, &self);

            match target.finish() {
                Ok(_) => (),
                Err(e) =>  match e {
                    SwapBuffersError::ContextLost => {
                        if let Some(mut callbacks) = take(&mut self.callbacks) {
                            callbacks.on_error(self, WindowRuntimeError::ContextLost);
                            self.callbacks = Some(callbacks);
                        }
                        //TODO: recreate context instead of crashing
                        todo!();
                    }
                    SwapBuffersError::AlreadySwapped => panic!("Buffers swapped multiple times - was target.finish called more than once?"),
                }
            }
            self.scene = Some(scene);
        }

        self.frame.input.scancodes_this_frame.clear();
        self.frame.input.keycodes_this_frame.clear();

    }

    ///Runs the window's event loop
    ///Can only be called once on a given window
    pub fn main_loop (mut self) -> Result<(), ()> {

        if self.event_loop_started {return Err(())}
        //Move event loop and callbacks out of the window object, so that functions on them can be called with window as an argument
        let event_loop = match self.event_loop {
            Some(ev) => ev,
            None => {return Err(())}
        };
        
        self.event_loop = None;

        event_loop.run(move |ev, _, control_flow| {

            if !self.event_loop_started {
                self.tick(control_flow);
                self.event_loop_started = true;
            }

            //Handle requested actions
            match self.run_actions() {
                Ok(()) => (),
                Err(()) => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
            }

            //Handle events
            match ev {

                Event::WindowEvent { event, .. } => match event {
                    //Call close callback on x press
                    glutin::event::WindowEvent::CloseRequested => {
                        println!("Close requested");
                        if let Some(mut callbacks) = take(&mut self.callbacks) {
                            callbacks.on_close(&mut self);
                            self.callbacks = Some(callbacks);
                        }
                    },
                    glutin::event::WindowEvent::Resized(ps) => {
                        if let Some(mut callbacks) = take(&mut self.callbacks) {
                            callbacks.on_resize(&mut self, Resolution::Physical(ps.width, ps.height));
                            self.callbacks = Some(callbacks);
                        }
                    },
                    glutin::event::WindowEvent::Moved(pp) => {
                        if let Some(mut callbacks) = take(&mut self.callbacks) {
                            callbacks.on_move(&mut self, Position::Physical(pp.x, pp.y));
                            self.callbacks = Some(callbacks);
                        }
                    }
                    //_ => {println!("Unhandled WindowEvent"); return},
                    _ => ()
                },
                Event::DeviceEvent { device_id: _, event } => {  
                    //println!("{:?} ", device_id);
                    match event {
                        
                        //glutin::event::DeviceEvent::MouseMotion { delta } => println!("Mouse motion: {:?}", delta),
                        //glutin::event::DeviceEvent::MouseWheel { delta } => println!("Mouse wheel: {:?}", delta),
                        //glutin::event::DeviceEvent::Motion { axis, value } => println!("Motion: {:?} by {:?}", axis, value),
                        //glutin::event::DeviceEvent::Button {button, state} => println!("Button: {:?} is {:?}", button, state),
                        glutin::event::DeviceEvent::Key(input) => {
                            self.frame.input.scancodes.insert(input.scancode, input.state == ElementState::Pressed);
                            self.frame.input.scancodes_this_frame.insert(input.scancode, input.state == ElementState::Pressed);
                            if let Some(keycode) = input.virtual_keycode {
                                self.frame.input.keycodes.insert(keycode, input.state == ElementState::Pressed);
                                self.frame.input.keycodes_this_frame.insert(keycode, input.state == ElementState::Pressed);
                            }
                        },
                        glutin::event::DeviceEvent::Text { codepoint } => println!("Text: {:?}", codepoint),
                        
                        _ => ()
                    }
                },
                Event::NewEvents(c) => match c {
                    StartCause::ResumeTimeReached { start: _, requested_resume: _ } => {
                        self.tick(control_flow);
                        if *control_flow == glutin::event_loop::ControlFlow::Exit {
                            return;
                        }
                    },
                    _ => ()
                },
                Event::MainEventsCleared => {},
                /* 
                Event::RedrawRequested(_) => println!("Redraw Requested"),
                Event::UserEvent(_) => println!("User event"),
                Event::Suspended => println!("Suspended"),
                Event::Resumed => println!("Resumed"),
                Event::RedrawEventsCleared => (),//println!("Redraw Events Cleared"),
                Event::LoopDestroyed => println!("Loop Destroyed")
                */
                _ => ()
            }
        });
    }
}