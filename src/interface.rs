use std::collections::BTreeMap;
pub use crate::utils::{Element, document, ElementResult};
use crate::render::Renderables;

pub type Hooks = BTreeMap<String, Element>;

trait A{
    fn html(&self)->String;
}

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
