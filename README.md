# 📋 ToDo CLI - Gestor de Tareas en Terminal

Un gestor de tareas simple y elegante desarrollado en Rust para la línea de comandos.

## 🚀 Características

- ✅ Añadir tareas nuevas
- 📝 Listar todas las tareas con colores
- ✓ Marcar tareas como completadas
- 🗑️ Eliminar tareas
- 🧹 Limpiar todas las tareas
- 💾 Persistencia automática en JSON
- 🎨 Interfaz colorida y amigable
- 📅 Timestamps de creación y finalización

## 🛠️ Tecnologías Utilizadas

- **Rust 1.89.0**
- **clap** - Parser de argumentos CLI
- **serde & serde_json** - Serialización JSON
- **chrono** - Manejo de fechas y timestamps
- **colored** - Salida colorizada en terminal

## 📂 Estructura del Proyecto

```
todo-cli/
│── Cargo.toml
│── README.md
└── src/
    ├── main.rs         // Punto de entrada
    ├── cli.rs          // Parser de argumentos con clap
    ├── task.rs         // Definición de la struct Task
    ├── storage.rs      // Persistencia en JSON
    └── commands.rs     // Implementación de comandos
```

## 📦 Instalación

1. Clona o crea el proyecto:

```bash
cargo new todo-cli
cd todo-cli
```

2. Copia todos los archivos del proyecto

3. Compila el proyecto:

```bash
cargo build --release
```

## 🎯 Uso

### Añadir una tarea

```bash
cargo run -- add "Aprender Rust"
cargo run -- add "Escribir documentación"
```

### Listar todas las tareas

```bash
cargo run -- list
```

Salida esperada:

```
📋 Task List:

[ ] 1 - Aprender Rust
    Created: 2024-01-15 10:30

[x] 2 - Escribir documentación
    Created: 2024-01-15 10:31 | Completed: 2024-01-15 11:45

📊 Summary:
Total: 2 | Completed: 1 | Pending: 1
```

### Marcar tarea como completada

```bash
cargo run -- done 1
```

### Eliminar una tarea

```bash
cargo run -- delete 1
```

### Limpiar todas las tareas

```bash
cargo run -- clear
```

### Ver ayuda

```bash
cargo run -- --help
```

## 🎨 Características de la Interfaz

- **Colores intuitivos**: Verde para completadas, rojo para pendientes
- **Iconos visuales**: Checkboxes, emojis y símbolos
- **Timestamps**: Fechas de creación y finalización
- **Resumen estadístico**: Total, completadas y pendientes
- **Texto tachado**: Para tareas completadas

## 📄 Formato del archivo JSON

Las tareas se guardan en `tasks.json`:

```json
{
  "tasks": [
    {
      "id": 1,
      "description": "Aprender Rust",
      "done": false,
      "created_at": "2024-01-15T10:30:00.000Z",
      "completed_at": null
    },
    {
      "id": 2,
      "description": "Escribir README",
      "done": true,
      "created_at": "2024-01-15T10:31:00.000Z",
      "completed_at": "2024-01-15T11:45:00.000Z"
    }
  ],
  "next_id": 3
}
```

## 🧪 Ejemplos de Uso

```bash
# Flujo típico de uso
cargo run -- add "Comprar leche"
cargo run -- add "Estudiar para el examen"
cargo run -- add "Hacer ejercicio"

cargo run -- list

cargo run -- done 2
cargo run -- list

cargo run -- delete 1
cargo run -- list

cargo run -- clear
```

## 🔧 Compilación y Distribución

### Compilar para producción:

```bash
cargo build --release
```

El binario estará en `target/release/rust-cli-task-manager`

### Instalar globalmente:

```bash
cargo install --path .
```

Después podrás usar directamente:

```bash
todo-cli add "Nueva tarea"
todo-cli list
```

## 📝 Licencia

Este proyecto está bajo la licencia MIT.

---

**¡Disfruta gestionando tus tareas con estilo! 🎉**
