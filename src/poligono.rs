use crate::framebuffer::FrameBuffer;
use crate::linea::Linea;

pub struct Poligono {
    vertices: Vec<(i32, i32)>,
}

impl Poligono {
    pub fn new(vertices: Vec<(i32, i32)>) -> Self {
        Poligono { vertices }
    }

    pub fn vertices(&self) -> &[(i32, i32)] {
        &self.vertices
    }

    pub fn borde(&self, fb: &mut FrameBuffer) {
        let n = self.vertices.len();
        for i in 0..n {
            let (x0, y0) = self.vertices[i];
            let (x1, y1) = self.vertices[(i + 1) % n];
            Linea::new(x0, y0, x1, y1).bresenham(fb);
        }
    }
}

struct Arista {
    y_min: i32,
    y_max: i32,
    x_en_y_min: f32,
    inv_pendiente: f32,
}

/// Rellena un polígono usando scanline con la regla par-impar.
/// Cada elemento de `contornos` es el borde de una figura cerrada; si se
/// pasa más de un contorno (p. ej. un polígono y un agujero adentro),
/// las áreas cubiertas por un número par de contornos quedan sin pintar.
pub fn rellenar(fb: &mut FrameBuffer, contornos: &[&[(i32, i32)]]) {
    let mut aristas = Vec::new();

    for contorno in contornos {
        let n = contorno.len();
        for i in 0..n {
            let (x0, y0) = contorno[i];
            let (x1, y1) = contorno[(i + 1) % n];
            if y0 == y1 {
                continue;
            }
            let (xa, ya, xb, yb) = if y0 < y1 {
                (x0, y0, x1, y1)
            } else {
                (x1, y1, x0, y0)
            };
            let inv_pendiente = (xb - xa) as f32 / (yb - ya) as f32;
            aristas.push(Arista {
                y_min: ya,
                y_max: yb,
                x_en_y_min: xa as f32,
                inv_pendiente,
            });
        }
    }

    let Some(y_min) = aristas.iter().map(|a| a.y_min).min() else {
        return;
    };
    let y_max = aristas.iter().map(|a| a.y_max).max().unwrap();

    for y in y_min..y_max {
        let mut cortes: Vec<f32> = aristas
            .iter()
            .filter(|a| y >= a.y_min && y < a.y_max)
            .map(|a| a.x_en_y_min + (y - a.y_min) as f32 * a.inv_pendiente)
            .collect();

        cortes.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for par in cortes.chunks_exact(2) {
            let x_inicio = par[0].round() as i32;
            let x_fin = par[1].round() as i32;
            for x in x_inicio..=x_fin {
                fb.point(x, y);
            }
        }
    }
}
