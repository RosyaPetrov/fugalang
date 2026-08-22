
<table>
<tr>
<td width="250">

<img src="docs/assets/fuga.png" width="250">

</td>

<td>

# fuga

**Современный компилируемый язык программирования.**

fugalang создаётся с упором на:

- простоту;
- производительность;
- статическую типизацию с выводом типов;
- выразительный лаконичный синтаксис.
- удобная работа с памятью

</td>
</tr>
</table>

<p align="center">
  <img src="https://img.shields.io/badge/status-development-34ebc9?style=flat-square">
  <img src="https://img.shields.io/badge/language-Rust-34ebc9?style=flat-square">
  <img src="https://img.shields.io/badge/license-Gpl-34ebc9?style=flat-square">
</p>

---

## Содержание

- [Установка](#установка)
- [Примеры](#примеры)
- [Лицензия](#лицензия)

## Установка

Для установки fuga выполните следующие команды в терминале

```bash
git clone https://github.com/fugalang/fugu.git
cd fugu
make install
```

> [!WARNING]
> Перед установкой убедитесь, что у вас установлены:
> - `git`
> - `rust`
> - `make`

## Примеры

```rust
fn main() {
    print("Hello fuga!")
}
```

[*больше примеров кода*](docs/exemples/README.md)

## Лицензия

fugalang распространяется под лицензией GPL 3

Подробности находятся в файле [LICENSE](LICENSE)