#![feature(vec_remove_item)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use std::fmt::Debug;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!("{:?}", $( $t )* ).into()))
}

//traits
trait Createable<T> {
    fn new() -> T;
}

trait Collectable {
    fn get_collection_mut(&mut self) -> &mut Collection;
    fn add(&mut self, obj: Box<ObjectTR>) -> bool {
        self.get_collection_mut().objects.push(obj);
        true
    }
    fn remove(&mut self, obj: Box<ObjectTR>) -> bool {
        self.get_collection_mut().objects.remove_item(&obj);
        true
    }
}

trait ObjectTR: Debug {
    fn id(&self) -> u32;
    fn get_object_mut(&mut self) -> &mut Object;
    fn set_position(&mut self, point: Point) {
        self.get_object_mut().point = point
    }
    fn render(&self, _context: &web_sys::CanvasRenderingContext2d);
}

impl PartialEq for ObjectTR {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

//trait Colorable {
//    fn get_color_mut(&mut self) -> &mut String;
//    fn set_color(&mut self, color: String) {
//        self.get_color_mut() = color;
//    }
//}

// primitives
#[derive(Debug)]
struct Point {
    left: f64,
    right: f64,
}

//objects
#[derive(Debug)]
struct Object {
    point: Point,
    opacity: f64,
}

impl Createable<Object> for Object {
    fn new() -> Object {
        Object {
            opacity: 0.3,
            point: Point {
                left: 100.0,
                right: 100.0,
            },
        }
    }
}

#[derive(Debug)]
struct Rect {
    id: u32,
    object: Object,
    width: f64,
    height: f64,
}


impl ObjectTR for Rect {
    fn id(&self) -> u32 {
        self.id
    }
    fn get_object_mut(&mut self) -> &mut Object {
        &mut self.object
    }
    fn render(&self, _context: &web_sys::CanvasRenderingContext2d) {
        _context.set_global_alpha(self.object.opacity);
        _context.rect(self.object.point.left, self.object.point.right, self.width, self.height);
        _context.fill();
    }
}

impl Createable<Rect> for Rect {
    fn new() -> Rect {
        Rect {
            id: 123,
            width: 50.0,
            height: 50.0,
            object: Object::new(),
        }
    }
}


//namespaces
#[derive(Debug)]
struct Collection {
    objects: Vec<Box<ObjectTR>>
}


// main
#[derive(Debug)]
struct StaticCanvas {
    canvas: web_sys::HtmlCanvasElement,
    context: web_sys::CanvasRenderingContext2d,
    collection: Collection,
}

impl StaticCanvas {
    fn render_all(&self) {
        self.context.clear_rect(0f64, 0f64, 1000f64, 1000f64);
        console_log!("clear");
        for item in self.collection.objects.iter() {
            item.render(&self.context);
        };
    }
}

impl Collectable for StaticCanvas {
    fn get_collection_mut(&mut self) -> &mut Collection {
        &mut self.collection
    }
}

impl Createable<StaticCanvas> for StaticCanvas {
    fn new() -> StaticCanvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas_el = document.get_element_by_id("canvas").unwrap();
        let canvas = canvas_el.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        StaticCanvas {
            canvas,
            context,
            collection: Collection { objects: vec![] },
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    let mut canvas = StaticCanvas::new();
    canvas.add(Box::new(Rect::new()));
    canvas.collection.objects[0].set_position(Point {
        left: 150.0,
        right: 150.0,
    });
    canvas.render_all();
    canvas.collection.objects[0].set_position(Point {
        left: 100.0,
        right: 100.0,
    });
    canvas.render_all();
    canvas.collection.objects[0].set_position(Point {
        left: 50.0,
        right: 50.0,
    });
    canvas.render_all();
}
