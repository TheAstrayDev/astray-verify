<?php
/**
 * Site content — The Astray
 */

$site = [
    'name' => 'The Astray',
    'title' => 'The Astray — Minecraft-плагины, Rust CLI и backend',
    'tagline' => 'Minecraft-плагины, Rust/forensics CLI, backend и разработка под заказ.',
    'email' => 'hello@theastraydev.online',
    'year' => (int) date('Y'),
    'telegram' => 'https://t.me/Jkkaall',
    'telegram_user' => '@Jkkaall',
];

$roles = [
    [
        'key' => 'programmer',
        'label' => 'Программист',
        'kicker' => 'код · системы · продукт',
        'text' => 'Пишу production-код на Rust, Java, PHP и Python. Minecraft-плагины с баллистикой, CLI-утилиты, бэкенд и инструменты, которыми реально пользуются.',
        'stack' => ['Rust', 'Java', 'PHP', 'Python', 'Paper/Fabric'],
    ],
    [
        'key' => 'engineer',
        'label' => 'Инженер',
        'kicker' => 'железо · протоколы · разбор',
        'text' => 'Разбираю USB/SCSI, протоколы устройств, passive-анализ захватов. Инженерный взгляд: не «магия», а измеримые системы и понятные ограничения.',
        'stack' => ['USB/SCSI', 'pcapng', 'Linux', 'Reverse-friendly', 'CLI/TUI'],
    ],
];

$projects = [
    [
        'slug' => 'realistic-guns',
        'title' => 'RealisticGuns',
        'category' => 'Minecraft · Paper',
        'year' => '2026',
        'summary' => 'Paper-плагин для Minecraft с настраиваемыми снарядами, пробитием, зонами попадания и CS2-style HUD.',
        'cover' => '/assets/img/projects/realistic-guns.svg',
        'accent' => '#1a1a1a',
        'body' => [
            'RealisticGuns — высоконастраиваемое огнестрельное оружие для Minecraft Paper 1.20.6 и 1.21. Плагин рассчитан на survival/RPG/мини-игры, где нужна честная баллистика, а не hitscan «из коробки».',
            'Пули — физические снаряды: гравитация, сопротивление воздуха, пробитие мягких блоков. Можно тюнить скорость, урон, разброс и поведение через YAML-конфиг без перекомпиляции.',
            'Зоны попадания (голова / тело / руки / ноги), трассеры, следы на блоках и Action Bar HUD боеприпасов. Подходит серверам, которым нужен кастомный gun-плагин с предсказуемой физикой.',
            'Стек: Java, Paper API, YAML. Заказ доработок и установка — через Telegram @Jkkaall.',
        ],
        'stack' => ['Java', 'Paper API', 'YAML config', 'Minecraft 1.20', 'Minecraft 1.21'],
        'link' => null,
        'seo_title_key' => null,
    ],
    [
        'slug' => 'both-invalid',
        'title' => 'BothInvalid',
        'category' => 'Minecraft · Fabric',
        'year' => '2025',
        'summary' => 'Кооперативный Fabric-мод: два игрока делят одного персонажа, но получают разные роли.',
        'cover' => '/assets/img/projects/both-invalid.svg',
        'accent' => '#111',
        'body' => [
            'BothInvalid — нестандартный coop-мод для Minecraft (Fabric): один аватар, два сознания. Исполнитель двигается и ломает блоки, но не видит мир.',
            'Штурман управляет камерой и инвентарём, видя глазами Исполнителя. Доверие между игроками становится игровой механикой.',
            'Сетевой слой синхронизирует ввод и камеру без «магических» костылей. Подходит для стримов, puzzle-карт и экспериментальных серверов.',
            'Стек: Java, Fabric API, custom networking. Вопросы и заказы модов — @Jkkaall.',
        ],
        'stack' => ['Java', 'Fabric', 'Networking', 'Minecraft mod'],
        'link' => null,
    ],
    [
        'slug' => 'usb-cdb-hunter',
        'title' => 'usb-cdb-hunter',
        'category' => 'Rust · Forensics',
        'year' => '2026',
        'summary' => 'Rust CLI для digital forensics: passive-анализ USB Mass Storage SCSI CDB из .pcapng без касания устройства.',
        'cover' => '/assets/img/projects/usb-cdb-hunter.svg',
        'accent' => '#0d3d3a',
        'body' => [
            'usb-cdb-hunter — утилита на Rust для разбора USB Mass Storage (BOT) и SCSI CDB из захватов Wireshark / USBPcap (.pcapng). Устройство не подключается: только файловый анализ.',
            'Находит BOT CBW, классифицирует 90+ SCSI opcode, подсвечивает vendor-unknown команды, ASCII-hints, JSON-вывод и diff двух захватов.',
            'Полезна для forensics, reverse-инженерии протоколов накопителей и обучения USB/SCSI без риска для железа.',
            'Стек: Rust, pcapng, SCSI/BOT. Кастомные forensics-CLI — обсуждаем в Telegram.',
        ],
        'stack' => ['Rust', 'pcapng', 'SCSI/BOT', 'digital forensics', 'CLI'],
        'link' => null,
    ],
    [
        'slug' => 'astray-verify',
        'title' => 'Astray Verify',
        'category' => 'Rust · MCP · CI',
        'year' => '2026',
        'summary' => 'Open-source CLI: записывает MCP-контракт и ловит сломанные tools, схемы и JSON-RPC до релиза.',
        'cover' => '/assets/img/projects/astray-verify.svg',
        'accent' => '#102121',
        'body' => [
            'Astray Verify превращает интерфейс MCP-сервера в проверяемый контракт. Один раз записывается рабочий tools/list, а затем команда test сравнивает сервер с сохранённой fixture.',
            'Инструмент проверяет stdio-handshake, список инструментов, JSON-схемы и чистоту JSON-RPC stdout. Если после изменений клиент потерял tool или схема стала несовместимой, CI завершится с понятной ошибкой.',
            'Это небольшой local-first инструмент на Rust: без модели, аккаунта и облака. Fixture хранится рядом с исходниками и ревьюится как обычное API-изменение.',
        ],
        'stack' => ['Rust', 'MCP', 'JSON-RPC', 'CI', 'Contract testing'],
        'link' => 'https://github.com/TheAstrayDev/astray-verify',
        'link_label' => 'Открыть GitHub',
    ],
];

$partners = [
    [
        'name' => 'BuiltByBit',
        'role' => 'Marketplace',
        'note' => 'Публикация и дистрибуция Minecraft-продуктов.',
        'url' => 'https://builtbybit.com/',
    ],
    [
        'name' => 'SprintHost',
        'role' => 'Infrastructure',
        'note' => 'Хостинг и инфраструктура theastraydev.online.',
        'url' => 'https://sprinthost.ru/',
    ],
    [
        'name' => 'Insolvo',
        'role' => 'Freelance',
        'note' => 'Заказы и продуктовая разработка на заказ.',
        'url' => 'https://insolvo.com/',
    ],
    [
        'name' => 'Open Source',
        'role' => 'Community',
        'note' => 'Инструменты и утилиты в открытом доступе.',
        'url' => null,
    ],
];

$offers = [
    [
        'id' => 'software',
        'title' => 'Купить software',
        'lead' => 'Готовые продукты с поддержкой и обновлениями.',
        'points' => [
            'Minecraft-плагины и моды',
            'CLI / desktop-утилиты',
            'Лицензия и документация',
            'Фиксы в рамках поддержки',
        ],
        'cta' => 'Смотреть продукты',
        'href' => '#contact',
        'tone' => 'dark',
    ],
    [
        'id' => 'custom',
        'title' => 'Заказать под ключ',
        'lead' => 'С нуля под вашу задачу: ТЗ → прототип → релиз.',
        'points' => [
            'Плагины, бэкенд, CLI, forensics-tooling',
            'Сроки и этапы прозрачно',
            'Исходники по договорённости',
            'Сопровождение после сдачи',
        ],
        'cta' => 'Написать brief',
        'href' => 'https://t.me/Jkkaall',
        'tone' => 'paper',
    ],
];

function project_by_slug(string $slug): ?array
{
    global $projects;
    foreach ($projects as $p) {
        if ($p['slug'] === $slug) {
            return $p;
        }
    }
    return null;
}
