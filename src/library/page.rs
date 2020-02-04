use crate::size::Size;
use crate::style::{Paper, PaperClass};
use super::*;


function! {
    /// `page.size`: Set the size of pages.
    #[derive(Debug, Clone, PartialEq)]
    pub struct PageSizeFunc {
        paper: Option<Paper>,
        extents: AxisMap<Size>,
        flip: bool,
    }

    parse(header, body, ctx, f) {
        body!(nope: body, f);
        PageSizeFunc {
            paper: header.args.pos.get::<Paper>(&mut f.errors),
            extents: AxisMap::parse::<ExtentKey>(&mut f.errors, &mut header.args.key),
            flip: header.args.key.get::<bool>(&mut f.errors, "flip").unwrap_or(false),
        }
    }

    layout(self, ctx, f) {
        let mut style = ctx.style.page;

        if let Some(paper) = self.paper {
            style.class = paper.class;
            style.dimensions = paper.dimensions;
        } else {
            style.class = PaperClass::Custom;
        }

        let map = self.extents.dedup(&mut f.errors, ctx.axes);
        map.with(Horizontal, |&width| style.dimensions.x = width);
        map.with(Vertical, |&height| style.dimensions.y = height);

        if self.flip {
            style.dimensions.swap();
        }

        vec![SetPageStyle(style)]
    }
}

function! {
    /// `page.margins`: Sets the page margins.
    #[derive(Debug, Clone, PartialEq)]
    pub struct PageMarginsFunc {
        padding: PaddingMap,
    }

    parse(header, body, ctx, f) {
        body!(nope: body, f);
        PageMarginsFunc {
            padding: PaddingMap::parse(&mut f.errors, &mut header.args),
        }
    }

    layout(self, ctx, f) {
        let mut style = ctx.style.page;
        self.padding.apply(&mut f.errors, ctx.axes, &mut style.margins);
        vec![SetPageStyle(style)]
    }
}
