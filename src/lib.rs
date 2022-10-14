pub mod render;
pub mod escape;
pub mod utils;
pub mod interface;
pub use interface::{Hooks, Html};

pub use workflow_html_macros::{html, tree, html_str, renderable};
pub use render::{Render, Renderables, Result, Write};
pub use escape::{escape_attr, escape_html};
use std::collections::BTreeMap;
pub use utils::{Element as WebElement, document, ElementResult};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone)]
pub enum AttributeValue{
    Bool(bool),
    Str(String)
}

#[derive(Debug, Default, Clone)]
pub struct Element<T:Render>{
    pub is_fragment:bool,
    pub tag:String,
    pub attributes:BTreeMap<String, AttributeValue>,
    pub children:Option<T>,
    pub reff:Option<(String, String)>,
    pub onclick : Arc<Mutex<Option<Closure::<dyn FnMut(web_sys::MouseEvent)>>>>
}

impl<T:Render+Clone+'static> Element<T>{
    pub fn on(self, name:&str, cb:Box<dyn Fn()>)->Self{
        if name.eq("click"){
            let mut onclick = self.onclick.lock().unwrap();
            *onclick = Some(Closure::<dyn FnMut(web_sys::MouseEvent)>::new(Box::new(move |_event: web_sys::MouseEvent| {
                cb()
            })));
        }
        self
    }
    //self_.home_item.element.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
}

pub trait ElementDefaults {
    fn _get_attributes(&self)->String;
    fn _get_children(&self)->String;

    fn get_attributes(&self)->String{
        self._get_attributes()
    }
    fn get_children(&self)->String{
        self._get_children()
    }
}

impl<T:Render+Clone+'static> Render for Element<T>{
    fn render_node(
        self,
        parent:&mut WebElement,
        map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>{
        renderables.push(Arc::new(self.clone()));
        let mut el = document()
        .create_element(&self.tag)?;
    
        let onclick = self.onclick.lock().unwrap();
        if let Some(onclick) = onclick.as_ref(){
            el.add_event_listener_with_callback("click", onclick.as_ref().unchecked_ref())?;
        }

        for (key, value) in &self.attributes{
            match value{
                AttributeValue::Bool(v)=>{
                    if *v {
                        el.set_attribute(key, "true")?;
                    }
                }
                AttributeValue::Str(v)=>{
                    el.set_attribute(key, v)?;
                }
            }
        }
        if let Some((key, value)) = self.reff{
            el.set_attribute("data-ref", &value)?;
            map.insert(key.to_string(), el.clone());
        }
        if let Some(children) = self.children{
            children.render_node(&mut el, map, renderables)?;
        }

        parent.append_child(&el)?;
        Ok(())
    }
    fn render(&self, w:&mut Vec<String>)->ElementResult<()>{
        if self.is_fragment{
            if let Some(children) = &self.children{
                children.render(w)?;
            }
        }else{
            w.push(format!("<{}", self.tag));
            for (key, value) in &self.attributes{
                match value{
                    AttributeValue::Bool(v)=>{
                        if *v {
                            w.push(format!(" {}", key));
                        }
                    }
                    AttributeValue::Str(v)=>{
                        w.push(format!(" {}=\"{}\"", key, (*v)));
                    }
                }
            }
            w.push(">".to_string());
            if let Some(children) = &self.children{
                children.render(w)?;
            }
            w.push(format!("</{}>", self.tag));
        }
        Ok(())
    }
}


#[cfg(test)]
mod test{
    //cargo test -- --nocapture --test-threads=1
    use crate::tree;
    use crate as workflow_html;
    use crate::Render;
    //use crate::renderable;
    //use crate::ElementDefaults;
    #[test]
    pub fn simple_html(){
        self::print_hr("simple_html");
        let tree = tree!{
            <p>
                <div class="xyz abc active">"some inner html"</div>
                <div class={"abc"}></div>
            </p>
        };
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(result, "<p><div class=\"xyz abc active\">some inner html</div><div class=\"abc\"></div></p>");
    }
    #[test]
    pub fn custom_elements(){
        self::print_hr("simple_html");
        let tree = tree!{
            <flow-select>
                <flow-menu-item class={"xyz"} />
                <flow-menu-item class={"abc"} />
            </flow-select>
        };
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(result, "<flow-select><flow-menu-item class=\"xyz\"></flow-menu-item><flow-menu-item class=\"abc\"></flow-menu-item></flow-select>");
    }
    #[test]
    pub fn without_root_element(){
        self::print_hr("without_root_element");
        let tree = tree!{
            <div class={"xyz"}></div>
            <div class={"abc"}></div>
        };
        let result = tree.html();
        println!("html: {}", result);
        assert_eq!(result, "<div class=\"xyz\"></div><div class=\"abc\"></div>");
    }
    #[test]
    pub fn complex_html(){
        self::print_hr("complex_html");
        /*let world  = "world";
        let num  = 123;
        let string  = "123".to_string();
        let string2  = "string2 value".to_string();
        let user = "123";
        let active = true;
        let disabled = false;
        let selected = "1";
        
        
        #[renderable(flow-select)]
        #[allow(unused_variables)]
        struct FlowSelect{
            #[attr(name="is-active")]
            pub active:bool,
            pub selected:String,
            pub name:String,
            pub children:Option<Vec<std::sync::Arc<dyn Render>>>,
            pub label:Option<String>
        }
        
        #[renderable(flow-menu-item)]
        struct FlowMenuItem<'a, R:Render>{
            pub text:&'a str,
            pub value:&'a str,
            pub children:Option<R>
        }


        //overries
        /*
        impl<'a> FlowSelect<'a>{
            
            fn get_attributes(&self)->String{
                format!("class=\"xxxxxxx\" active")
            }
            fn get_children(&self)->String{
                format!("<flow-menu-item value=\"sss\">xyz</flow-menu-item>")
            }
        }
        */
        //let name = "abc".to_string();
        //let selected = "1".to_string();
        let name2 = "aaa".to_string();
        let name3 = "bbb".to_string();
        let tree = tree!{
            <div class={"abc"} ?active ?disabled ?active2={false} user data-user-name={"test-node"} &string2>
                {123} {"hello"} {world} {num} {num} {num} {string} {true}
                {1.2 as f64}
                <h1>{"hello 123"} {num}</h1>
                {"10"}
                {11}
                {12} {13} {14}
                <h3>{"single child"}</h3>
                <FlowSelect active name={name2} selected={"<1&2>\"3"} />
                <div class={"abc"}></div>
                <FlowSelect active name={name3} &selected>
                    <flow text={"abc"} />
                    <FlowMenuItem text={"abc"} value={"abc"} />
                </FlowSelect>
            </div>
        };
        
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(
            result,
            "<div active class=\"abc\" data-user-name=\"test-node\" string2=\"string2 value\" user=\"123\">123helloworld123123123123true1.2<h1>hello 123123</h1>1011121314<h3>single child</h3><flow-select is-active selected=\"&lt;1&amp;2&gt;&quot;3\" name=\"aaa\"></flow-select><div class=\"abc\"></div><flow-select is-active selected=\"1\" name=\"bbb\"><flow text=\"abc\"></flow><flow-menu-item text=\"abc\" value=\"abc\"></flow-menu-item></flow-select></div>"
        );
        */
    }

    fn print_hr<'a>(_title: &'a str){
        //println!("\n☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁\n");
        println!("\n☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰\n")
    }
}
