mod framebuffer;
mod linea;
mod poligono;

use framebuffer::FrameBuffer;
use poligono::{rellenar, Poligono};
use raylib::prelude::*;

fn desplazado(vertices: &[(i32, i32)], dx: i32, dy: i32) -> Vec<(i32, i32)> {
    vertices.iter().map(|&(x, y)| (x + dx, y + dy)).collect()
}

fn main() {
    let width = 800;
    let height = 700;

    let poligono1 = Poligono::new(vec![
        (165, 380),
        (185, 360),
        (180, 330),
        (207, 345),
        (233, 330),
        (230, 360),
        (250, 380),
        (220, 385),
        (205, 410),
        (193, 383),
    ]);

    let poligono2 = Poligono::new(vec![(321, 335), (288, 286), (339, 251), (374, 302)]);

    let poligono3 = Poligono::new(vec![(377, 249), (411, 197), (436, 249)]);

    let poligono4 = Poligono::new(vec![
        (413, 177),
        (448, 159),
        (502, 88),
        (553, 53),
        (535, 36),
        (676, 37),
        (660, 52),
        (750, 145),
        (761, 179),
        (672, 192),
        (659, 214),
        (615, 214),
        (632, 230),
        (580, 230),
        (597, 215),
        (552, 214),
        (517, 144),
        (466, 180),
    ]);

    // Agujero dentro del polígono 4: no debe pintarse.
    let poligono5 = Poligono::new(vec![(682, 175), (708, 120), (735, 148), (739, 170)]);

    // Segunda copia del polígono 4 (con su agujero), desplazada hacia abajo
    // para que no se encime con la primera copia sólida.
    let desplazamiento_y = 260;
    let poligono4_con_hueco = Poligono::new(desplazado(poligono4.vertices(), 0, desplazamiento_y));
    let poligono5_hueco = Poligono::new(desplazado(poligono5.vertices(), 0, desplazamiento_y));

    let mut fb = FrameBuffer::new(width, height, Color::BLACK);

    fb.set_current_color(Color::RED);
    rellenar(&mut fb, &[poligono1.vertices()]);
    fb.set_current_color(Color::WHITE);
    poligono1.borde(&mut fb);

    fb.set_current_color(Color::GREEN);
    rellenar(&mut fb, &[poligono2.vertices()]);
    fb.set_current_color(Color::WHITE);
    poligono2.borde(&mut fb);

    fb.set_current_color(Color::BLUE);
    rellenar(&mut fb, &[poligono3.vertices()]);
    fb.set_current_color(Color::WHITE);
    poligono3.borde(&mut fb);

    // Polígono 4 sólido, sin agujero.
    fb.set_current_color(Color::YELLOW);
    rellenar(&mut fb, &[poligono4.vertices()]);
    fb.set_current_color(Color::WHITE);
    poligono4.borde(&mut fb);

    // Polígono 4 otra vez, ahora con el polígono 5 adentro como agujero: al
    // pasar ambos contornos juntos, la regla par-impar deja sin pintar el
    // área del 5.
    fb.set_current_color(Color::YELLOW);
    rellenar(
        &mut fb,
        &[poligono4_con_hueco.vertices(), poligono5_hueco.vertices()],
    );
    fb.set_current_color(Color::WHITE);
    poligono4_con_hueco.borde(&mut fb);
    poligono5_hueco.borde(&mut fb);

    fb.render("out.png");

    println!("Imagen guardada correctamente.");
}
