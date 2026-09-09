-- Corrections to the Simplified Chinese genre names seeded by the previous
-- zh translation migration. Applied as UPDATEs so that databases which already
-- ran that migration pick up the fixes.

UPDATE genre_translations SET name = '动作小说'          WHERE genre_id = 6  AND lang = 'zh';
UPDATE genre_translations SET name = '反讽侦探'          WHERE genre_id = 8  AND lang = 'zh';
UPDATE genre_translations SET name = '刑侦小说'          WHERE genre_id = 14 AND lang = 'zh';
UPDATE genre_translations SET name = '疯狂题材'          WHERE genre_id = 15 AND lang = 'zh';
UPDATE genre_translations SET name = '神秘剧、闹剧、轻歌舞剧' WHERE genre_id = 43 AND lang = 'zh';
UPDATE genre_translations SET name = '戏剧艺术'          WHERE genre_id = 56 AND lang = 'zh';
