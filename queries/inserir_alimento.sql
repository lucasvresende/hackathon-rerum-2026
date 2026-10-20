INSERT INTO alimentos(id, nome, classe) VALUES (?, ?, ?) ON CONFLICT(id) DO NOTHING;
