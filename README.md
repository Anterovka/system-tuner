<div align="center">

# ⚙ System Tuner

**Настройка Linux из одного окна**

</div>

---

## Что это?

**System Tuner** — графический интерфейс для настройки параметров Linux. Вместо десятков терминалов — одно окно с интуитивным интерфейсом.

<br>

## Возможности

### 🔧 Ядро (sysctl)
- Настройка swappiness, dirty_ratio, vfs_cache_pressure
- Оптимизация сетевых буферов (TCP, UDP)
- Scheduler параметры
- Изменения применяются мгновенно

### ⚡ Службы (systemd)
- Включение/отключение сервисов
- Запуск/остановка служб
- Фильтрация по имени
- Визуальный статус (active/enabled/inactive)

### 💾 Хранилище
- Управление swap и swappiness
- Информация о ZRAM
- TRIM для SSD (одна кнопка)
- Информация о дисках (SSD/HDD)

### 🚀 Производительность
- Переключение CPU governor
- Готовые профили: **Performance**, **Balanced**, **Powersave**
- Информация о CPU (модель, ядра, частота)

<br>

## Установка

### AppImage (рекомендуется)

```bash
# Скачиваем последний релиз
wget https://github.com/Anterovka/system-tuner/releases/latest/download/system-tuner.AppImage
chmod +x system-tuner.AppImage
sudo ./system-tuner.AppImage
```

### cargo install

```bash
cargo install --git https://github.com/Anterovka/system-tuner.git
sudo system-tuner
```

### Из исходников

```bash
git clone https://github.com/Anterovka/system-tuner.git
cd system-tuner
cargo build --release
sudo ./target/release/system-tuner
```

<br>

### Зависимости

- **Rust** 1.75+ (только при сборке из исходников)
- **sudo** — для записи в sysctl и управления systemd

<br>

## Поддержка дистрибутивов

| Дистрибутив | Статус |
|-------------|--------|
| Arch Linux | ✅ Полная поддержка |
| Fedora | ✅ Полная поддержка |
| Ubuntu / Debian | ✅ Полная поддержка |
| openSUSE | ✅ Полная поддержка |
| Manjaro | ✅ Полная поддержка |
| Linux Mint | ✅ Полная поддержка |
| Pop!_OS | ✅ Полная поддержка |
| Void Linux | ❌ Нет systemd |
| Alpine Linux | ❌ Минимальный systemd |

> **Требование**: Дистрибутив с **systemd**. Все параметры используют стандартные Linux интерфейсы (`/proc/sys`, `systemctl`).

<br>

## Структура проекта

```
system-tuner/
├── src/
│   ├── main.rs              # Точка входа
│   ├── app.rs               # Главное окно, навигация, тема
│   ├── backend/
│   │   ├── sysctl.rs        # Чтение/запись параметров ядра
│   │   ├── systemd.rs       # Управление службами
│   │   ├── storage.rs       # Swap, ZRAM, TRIM, диски
│   │   └── performance.rs   # CPU governor, профили
│   └── pages/
│       ├── kernel.rs        # Страница настроек ядра
│       ├── services.rs      # Страница служб
│       ├── storage.rs       # Страница хранилища
│       └── performance.rs   # Страница производительности
└── Cargo.toml
```

<br>

## Технологии

- **Rust** — безопасность и производительность
- **egui** — быстрый immediate-mode GUI
- **eframe** — кроссплатформенный фреймворк

<br>

## Contributing

PR приветствуются! Если хочешь добавить:
- Новые параметры sysctl
- Поддержку других DE/WM
- Экспорт/импорт профилей
- Автозапуск с настройками

<br>

## Лицензия

MIT

<br>

---

<div align="center">

**Сделано с ❤ для Linux сообщества**

</div>
