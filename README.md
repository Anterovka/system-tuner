<div align="center">

# ⚙ System Tuner

**Настройка Linux из одного окна**

</div>

---

## Что это?

**System Tuner** — графический интерфейс для настройки параметров Linux. 

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

## Технологии

- **Rust** — безопасность и производительность
- **egui** — быстрый immediate-mode GUI
- **eframe** — кроссплатформенный фреймворк

<br>

## Лицензия

MIT

<br>

---

<div align="center">

**Сделано с ❤ для Linux сообщества**

</div>
