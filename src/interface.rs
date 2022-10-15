use std::collections::BTreeMap;
pub use crate::utils::{Element, document, ElementResult};
use crate::render::{Render, Renderables};
use crate::WebElement;
pub type Hooks = BTreeMap<String, Element>;

#[derive(Clone)]
pub struct Html{
    pub roots: Vec<Element>,
    pub hooks: Hooks,
    pub renderables: Renderables
}

//pub type Html_ = Html<dyn Renderable>;

impl Html{
    pub fn new(
        roots : Vec<Element>,
        hooks : Hooks,
        renderables:Renderables
    ) -> ElementResult<Html> {
        let html = Html {
            roots,
            hooks,
            renderables
        };
        Ok(html)
    }

    pub fn roots<'html>(&'html self) -> &'html Vec<Element> {
        &self.roots
    }

    pub fn hooks<'html>(&'html self) -> &'html Hooks {
        &self.hooks
    }

    pub fn inject_into(&self, element : &Element) -> ElementResult<()> {
        for root in self.roots.iter() {
            element.append_child(&root)?;
        }
        Ok(())
    }
}


impl Render for Html{
    fn render_node(
        mut self,
        parent:&mut WebElement,
        map:&mut Hooks,
        renderables:&mut Renderables
    )->ElementResult<()>{
        renderables.append(self.renderables.as_mut());
        let mut hooks = self.hooks().clone();
        map.append(&mut hooks);
        self.inject_into(&parent)?;
        Ok(())
    }

    fn render(&self, _w:&mut Vec<String>)->ElementResult<()>{
        Ok(())
    }
}
