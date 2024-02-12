use ggez::{
    graphics::{Color, DrawMode, Drawable, Mesh, Rect},
    Context,
};

#[allow(unused)]
pub fn rect_mesh_set_rect(ctx: &Context, mesh: &mut Mesh, color: Color, new_rect: Rect) {
    *mesh = Mesh::new_rectangle(&ctx.gfx, DrawMode::fill(), new_rect, color).unwrap()
}

#[allow(unused)]
pub fn rect_mesh_set_x(ctx: &Context, mesh: &mut Mesh, color: Color, x: f32) {
    let rect = mesh.dimensions(&ctx.gfx).unwrap();

    rect_mesh_set_rect(
        ctx,
        mesh,
        color,
        Rect {
            x,
            y: rect.y,
            w: rect.w,
            h: rect.h,
        },
    );
}

#[allow(unused)]
pub fn rect_mesh_set_y(ctx: &Context, mesh: &mut Mesh, color: Color, y: f32) {
    let rect = mesh.dimensions(&ctx.gfx).unwrap();

    rect_mesh_set_rect(
        ctx,
        mesh,
        color,
        Rect {
            x: rect.x,
            y,
            w: rect.w,
            h: rect.h,
        },
    );
}

#[allow(unused)]
pub fn rect_mesh_set_w(ctx: &Context, mesh: &mut Mesh, color: Color, w: f32) {
    let rect = mesh.dimensions(&ctx.gfx).unwrap();

    rect_mesh_set_rect(
        ctx,
        mesh,
        color,
        Rect {
            x: rect.x,
            y: rect.y,
            w,
            h: rect.h,
        },
    );
}

#[allow(unused)]
pub fn rect_mesh_set_h(ctx: &Context, mesh: &mut Mesh, color: Color, h: f32) {
    let rect = mesh.dimensions(&ctx.gfx).unwrap();

    rect_mesh_set_rect(
        ctx,
        mesh,
        color,
        Rect {
            x: rect.x,
            y: rect.y,
            w: rect.w,
            h,
        },
    );
}
