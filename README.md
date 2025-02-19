# ¿Contraseña fuerte?

Para que nuestros datos estén seguros en redes sociales, cuentas de bancos y demás sitios web, necesitamos generar una contraseña fuerte. Pensemos que la contraseña es la primera capa de defensa ante un atacante (ciberdelincuente).

# ¿Cómo es una contraseña fuerte?

Una contraseña fuerte tiene que tener distintas combinaciones y de una longitud de 8 caracteres o más.

Puede estar compuesta por:

- Mayúsculas
- Minúsculas
- Símbolos
- Letras

## Código de ejemplo para generar contraseñas

Hice una pequeña herramienta básica en Rust para poder realizar la explicación y mostrar ejemplos, de cómo debe ser una contraseña segura y de cómo no debe ser.

Contraseñas solamente de números, letras o símbolos, son relativamente débiles y fáciles de descubrir.

### Numeros
```rust
fn numbers() -> String {
    let number: Vec<char> = "123456789".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *number.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}
```
**Ejemplo: 14117833877145556291**

### Letras: mayuscula y minuscula
```rust
fn letter() -> String {
    let letters: Vec<char> = "ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *letters.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}
```
**Ejemplo: nRTmaXWIbjQAtSHqPsQB**

### Simbolos
```rust
fn symbol() -> String {
    let symbols: Vec<char> = "!@#$%^&*()_+-=[]{}|;':,./<>?".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *symbols.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}

```
**Ejemplo: #/?;$-&|@+:/}#';-({@**

### Combinacion
Este tipo de contraseña es la recomendada ,es una combinacion de letras,numeros y simbolos,es mucho mas fuerte que solo poner letras o numeros.
```rust
fn combination() -> String {
    let password: Vec<char> = "!@#$%^&*()_+-=[]{}|;':,./<>?123456789ABCDEFGHIJKLMNÑOPQRSTUVWXYZabcdefghijklmnñopqrstuvwxyz".chars().collect();
    let  random: String = (0..LENGTH).map(|_|{
        *password.choose(&mut rand::thread_rng()).unwrap()
        }
    ).collect();
    random
}
```
**Ejemplo: G1}(FX2IocF[.'@zZnGp**
### Comprobar contraseña
* [Cómo crear una contraseña segura de NordPass](https://nordpass.com/es/secure-password/)
* [Cómo verificar la seguridad de tus contraseñas de RoboForm](https://www.roboform.com/es/how-secure-is-my-password/)
* [Verificador de contraseñas de Internxt](https://internxt.com/es/password-checker)

### Recursos
* [Cómo crear una contraseña segura de Google](https://support.google.com/accounts/answer/32040?hl=es-419)
* [Cómo crear una contraseña segura](https://latam.kaspersky.com/resource-center/threats/how-to-create-a-strong-password)
* [Cómo crear y usar contraseñas seguras](https://support.microsoft.com/es-es/windows/crear-y-usar-contrase%C3%B1as-seguras-c5cebb49-8c53-4f5e-2bc4-fe357ca048eb)

