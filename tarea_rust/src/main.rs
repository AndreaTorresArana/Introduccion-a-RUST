fn main() {
    println!("============================");
    println!("   EJEMPLOS BÁSICOS EN RUST ");
    println!("============================\n");

    // --------------------------
    // 1. Variables e inmutabilidad
    // --------------------------
    println!("1) Variables e inmutabilidad:");
    let x = 5;
    println!("x inicial (inmutable): {}", x);
    // x = 6; // ❌ Esto daría error porque es inmutable

    // --------------------------
    // 2. Mutabilidad
    // --------------------------
    println!("\n2) Mutabilidad:");
    let mut y = 10;
    println!("y inicial: {}", y);
    y = 20;
    println!("y actualizado: {}", y);

    // --------------------------
    // 3. Sombreado (Shadowing)
    // --------------------------
    println!("\n3) Sombreado:");
    let z = 2;
    let z = z + 3; // z = 5
    let z = "texto"; // ahora z es un string
    println!("z sombreado: {}", z);

    // --------------------------
    // 4. Constantes
    // --------------------------
    println!("\n4) Constantes:");
    const PI: f64 = 3.1416;
    println!("Constante PI: {}", PI);

    // --------------------------
    // 5. Tipos básicos
    // --------------------------
    println!("\n5) Tipos básicos:");
    let entero: i32 = 42;
    let flotante: f64 = 3.14;
    let booleano: bool = true;
    let caracter: char = 'L';
    println!("Entero: {}", entero);
    println!("Flotante: {}", flotante);
    println!("Booleano: {}", booleano);
    println!("Carácter: {}", caracter);

    // --------------------------
    // 6. Tipos compuestos
    // --------------------------
    println!("\n6) Tipos compuestos:");
    let tupla: (i32, f64, char) = (42, 6.28, 'A');
    let arreglo: [i32; 3] = [1, 2, 3];
    println!("Tupla completa: {:?}", tupla);
    println!("Segundo elemento de la tupla: {}", tupla.1);
    println!("Arreglo completo: {:?}", arreglo);
    println!("Primer valor del arreglo: {}", arreglo[0]);

    // --------------------------
    // 7. Operaciones básicas
    // --------------------------
    println!("\n7) Operaciones básicas:");
    let a = 15;
    let b = 4;
    println!("Suma: {}", a + b);
    println!("Resta: {}", a - b);
    println!("Multiplicación: {}", a * b);
    println!("División: {}", a / b);
    println!("Módulo: {}", a % b);
    println!("¿a > b?: {}", a > b);
    println!("¿a == b?: {}", a == b);
    println!("Lógica: true && false = {}", true && false);

    // --------------------------
    // 8. Formato de impresión
    // --------------------------
    println!("\n8) Formato de impresión:");
    let numero = 99;
    let tupla = (7, 3.14, 'Z');
    println!("Número con {{}}: {}", numero);
    println!("Tupla con debug {{:?}}: {:?}", tupla);
    println!("Tupla manual: ({}, {}, {})", tupla.0, tupla.1, tupla.2);

    println!("\n=== Fin de los ejemplos ===");
}
