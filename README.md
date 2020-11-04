# Edytor tekstu online

Edytor tekstu online umożliwiający współpracę wielu osób na raz nad folderem plików.

<b>Ważne</b>

Ta aplikacja jest antywzorcem bezpieczeństwa i nie powinno się w żadnym przypadku na niej wzorować.
Przechowywanie w ciasteczku numeru sesji oraz identyfikatora użytkownika podłączonego do sesji a w bazie używanie md5 jako hasha dla haseł to rozwiązanie
zastosowane jedynie dlatego, że celem aplikacji nie było zapewnienie bezpiecznego przechowywania plików a jedynie umożliwienie
równoległe ich edytowanie.


## Uruchamianie

```bash
docker-compose build --parallel
docker-compose up
```

Można również na maszynach z python 3(python używany jedynie do otwarcia automatycznie strony):
```bash
./build-and-run.sh
```
Ten skrypt zawiera istrukcje wpisane powyżej + komendę do otwarcia przeglądarki

Może sie w tym przypadku zdarzyć, że przeglądarka spróbuje załadować strone przed postawieniem dockera. Rozwiązanie to
po prostu przełądować stronę.

