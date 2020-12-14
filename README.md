# Edytor tekstu online

Edytor tekstu online umożliwiający współpracę wielu osób na raz nad folderem plików.

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

### Uwagi co do uruchamiania

Ze względów na sposób działania CORS w różnych przeglądarkach, ciasteczko zawierające identyfikator sesji(a w zasadzie
id użytkownika)  jest secure. Sprawia to, że apka działa w Firefoxie któ©y jeszcze wysyłą ciasteczka secure do localhost(ale nie do 127.0.0.1).

W chrome jednak nie działa dopóki się jakoś nie skonfiguruje HTTPS lub w inicjalizacji serwera w pliku main.rs nie zmieni

```rust
.secure(true)
```

na 

```rust
.secure(false)
```

## Bezpieczeństwo 

Ta aplikacja(na razie) jest antwzorcem bezpieczeństwa, poniważ celem nie było zapewnienie bezpiecznej
aplikacji a jedynie edytora umożliwiającego współpracę. 

Naruszenia:
- md5 użyte do hashowania haseł
- numer sesji przechowywany w ciasteczku, które można zmienić na stronie klienta(co prawda nie przez js ale wciąż)
- jeżeli w trakcie edycji projektu właściciel odbierze dostęp użytkownikowi edytującemu, nie zostanie on wylogowany. Będzie mógł
edytować istniejące pliki ale nie będzie mógł usuwać i tworzyć nowych.




