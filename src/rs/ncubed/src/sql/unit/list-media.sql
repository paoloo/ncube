SELECT id_hash,
       type,
       term,
       data
  FROM media
 WHERE unit = ?1;
