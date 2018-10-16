# <img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/assets/pyxel_logo_152x64.png">

[ [English](https://github.com/kitao/pyxel/blob/master/README.md) | [日本語](https://github.com/kitao/pyxel/blob/master/README.ja.md) | [Other Languages](https://github.com/kitao/pyxel/wiki) ]

**Pyxel (პიქსელი)** რეალიზებულია როგორც რეტრო თამაშთა ძრავი Python-ის პროგრამული ენისთვის.

ძრავის სპეციფიკებიდან გამომდინარე, რომლებიც შთაგონებულნი არიან რეტრო სათამაშო კონსოლების მიერ, Pyxel-ის ძრავა ბადებს შესაძლებლობას რომ მოხდეს აღნიშნული კონსოლების სიმულაცია. მაგალითად, მხოლოდ 16 ფერის გამოსახვის შესაძლებლობა ეკრანზე და მხოლოდ 4 სხვადასხვა ხმის გამომუშავება ერთდროულად. შედეგად, თქვენ შესაძლებლობა გეძლევათ, რომ უფრო მოხერხებულად შექმნათ პიქსელ არტის სტილის თამაშები.

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/01_hello_pyxel.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/01_hello_pyxel.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/02_jump_game.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/02_jump_game.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/03_draw_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/03_draw_api.gif" width="48%">
</a>

<a href="https://github.com/kitao/pyxel/blob/master/pyxel/examples/04_sound_api.py" target="_blank">
<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/04_sound_api.gif" width="48%">
</a>

სათამაშო კონსოლის სპეციფიკაციები, API-ები და Pyxel-ის პალიტრა წარმოშობილია [PICO-8](https://www.lexaloffle.com/pico-8.php)-ისა და [TIC-80](https://tic.computer/)-ის შთაგონებით.

Pyxel-ი უფასო და ღია კოდის ტიპის პროექტია. შეგვიძლია შევუდგეთ რეტრო თამაშის შექმნას Pyxel-ის გამოყენებით!

## სპეციფიკაციები

- მუშაობს Windows-ზე, Mac-ზე და Linux-ზე
- კოდს ვწერთ Python3-ში
- ფიქსირებული 16 ფერიანი პალიტრა
- 256x256 ზომის 3 გამოსახულების ბანკი
- 4 არხი 64 მსაზღვრელობითი ხმის ბანკით
- კლავიატურის, მაუსის და ჯოისტიკის (WIP) შენატანის მხარდაჭერა
- გამოსახულების და ხმის რედაქტორი (WIP)

### ფერთა პალიტრა

<img src="https://raw.githubusercontent.com/kitao/pyxel/master/pyxel/examples/screenshots/05_color_palette.png">

## დაყენების ინსტრუქციები

### Windows

[Python3](https://www.python.org/)-ის დაყენების შემდეგ, მომდევნო `pip` ბრძანება (ბრძანებათა ველში) აყენებს Pyxel-ს:

```sh
pip install pyxel
```

### Mac

[Python3](https://www.python.org/)-ის და [glfw](http://www.glfw.org/)-ს (ვერსიით 3.2.1 ან უფრო მაღალი) დაყენების შემდეგ, დააყენეთ Pyxel-ი მომდევნო `pip` ბრძანებით.

თუ [Homebrew](https://brew.sh/) შეფუთვის მენეჯერი დაყენებულია თქვენს სისტემაზე, მაშინ შეგიძლიათ მომდევნო ბრძანებებით დააყენოთ ყველა საჭირო ბიბლიოთეკა Pyxel-ის გასაშვებად:

```sh
brew install python3 glfw
pip3 install pyxel
```

### Linux

დააყენეთ საჭირო ფაილები, რომლებიც სპეციფიურნი არიან სხვადასხვა დისტრიბუტივებისთვის. [glfw](http://www.glfw.org/) უნდა იყოს 3.2.1 ან უფრო მაღალი ვერსიით გამოშვებული.

**Arch:**

დააყენეთ [`python-pixel`](https://aur.archlinux.org/packages/python-pyxel/) თქვენი ფავორიტი AUR-ის დამხმარის საშუალებით:

```sh
yay -S python-pyxel
```

**Debian:**

```sh
apt-get install python3 python3-pip libglfw3 libportaudio2 libasound-dev
pip3 install pyxel
```

**Fedora:**

```sh
dnf install glfw portaudio
pip3 install pyxel
```

### მაგალითების დაყენება

Pyxel-ის დაყენების შემდეგ, თქვენს საინსტალაციო დირექტორიაში შეგიძლიათ გააჩინოთ Pyxel ძრავით შექმნილი მაგალითები მომდევნო ბრძანებით:

```sh
install_pyxel_examples
```

## გამოყენების ინსტრუქციები

### შევქმნათ Pyxel-ის აპლიკაცია

პითონის კოდში Pyxel-ის მოდულის იმპორტირების შემდეგ, მიუთითეთ ფანჯრის ზომა `init` ფუნქციით და გაუშვით Pyxel-ის აპლიკაცია `run` ფუნქციით.

```python
import pyxel

pyxel.init(160, 120)

def update():
    if pyxel.btnp(pyxel.KEY_Q):
        pyxel.quit()

def draw():
    pyxel.cls(0)
    pyxel.rect(10, 10, 20, 20, 11)

pyxel.run(update, draw)
```

`run` ფუნქციის არგუმენტებია `update` ფუნქცია რომ განახლდეს თითოეული კადრი და `draw` ფუნქცია რომ დავხატოთ ეკრანზე როცა საჭირო იქნება.

ნამდვილ აპლიკაციაში რეკომენდირებულია, რომ Pyxel-ის კოდი კლასში მოვაქციოთ როგორც ნაჩვენებია ქვემოთ:

```python
import pyxel

class App:
    def __init__(self):
        pyxel.init(160, 120)
        self.x = 0
        pyxel.run(self.update, self.draw)

    def update(self):
        self.x = (self.x + 1) % pyxel.width

    def draw(self):
        pyxel.cls(0)
        pyxel.rect(self.x, 0, self.x + 7, 7, 9)

App()
```

### სპეციალური მართვის საშუალებები

ქვემოთ ჩამოთვლილი მართვის საშუალებები შესრულებული შეიძლება იქნას როდესაც Pyxel-ის აპლიკაცია გაშვებული იქნება:

- `Alt(Option)+1`  
სქრინშოთის ჩაწერა დესკტოპზე
- `Alt(Option)+2`  
ეკრანის ასახვითი ვიდეოს დაწყების დროის საწყის მდგომარეობაში დაბრუნება
- `Alt(Option)+3`  
ეკრანის ასახვითი ვიდეოს (gif) ჩაწერა დესკტოპზე (30 წამამდე)
- `Alt(Option)+0`  
შრომის ინტენსიურობის მონიტორის ჩართვა/გამორთვა (fps, განახლების დრო, დახატვის დრო)
- `Alt(Option)+Enter`  
სავსე ეკრანის ჩართვა/გამორთვა

### გამოსახულებების შექმნა

ქვემოთ ჩამოთვლილია საშუალებები Pyxel-ში გამოსახულებების შექმნისთვის:

- გამოსახულების შექმნა სტრიქონთა სიიდან `Image.set` ფუნქციის გამოყენებით
- png გაფართოების ფაილის ჩატვირთვა Pyxel-ის პალიტრაში `Image.load` ფუნქციის გამოყენებით
- გამოსახულებების შექმნა Pyxel Editor-ის გამოყენებით (WIP)

გთხოვთ გაეცნოთ API ცნობარს `Image.set` და `Image.load` ფუნქციების გამოყენებისთვის.

იმისდა მიხედვით, რომ Pyxel-ი იყენებს იგივე პალიტრას რაც [PICO-8](https://www.lexaloffle.com/pico-8.php), როდესაც png გაფართოების გამოსახულებებს ვქმნით Pyxel-ში, რეკომენდირებულია რომ გამოვიყენოთ [Aseprite](https://www.aseprite.org/) PICO-8-ის პალიტრის რეჟიმის გამოყენების დროს.

## API ცნობარი

### სისტემა

- `width`, `height`  
ეკრანის სიგანე და სიმაღლე

- `frame_count`  
გასული კადრების რაოდენობა

- `init(width, height, [caption], [scale], [palette], [fps], [border_width], [border_color])`  
Pyxel აპლიკაციის ინიციალიზება ეკრანის ზომით (`width`, `height`). ეკრანის მაქსიმალური სიგანე და სიმაღლე არის 255  
ასევე შესაძლებელი არის, რომ ფანჯრის ტიტული შევცვალოთ `caption`-ით, ჩვენების მაგნიფიკაცია `scale`-ით, პალიტრის ფერები `palette`-ით, კადრების სიხშირე `fps`-ით და მარჟის სიგანე და ფერი ეკრანის გარეთ `border_width` და `border_color`-ით. `palette` არის განსაზღვრული როგორც სია 16 ელემენტებისა 24 ბიტიანი ფერის, `border_color`-იც განისაზღვრება 24 ბიტიანი ფერით

- `run(update, draw)`  
ჩართვა Pyxel აპლიკაციის და გამოძახება `update` ფუნქციის რომ კადრები განახლდეს და `draw` ფუნქციის რომ ეკრანზე დაიხატოს

- `quit()`  
Pyxel აპლიკაციის გამორთვა უკანასკნელი კადრის ბოლოს

### რესურსი

- `save(filename)`  
ჩაწერა რესურს ფაილის (.pyxel) იმ დირექტორიაში, სადაც გაშვების სკრიპტი წერია

- `load(filename)`  
წაკითხვა რესურს ფაილის (.pyxel) იმ დირექტორიიდან, სადაც გაშვების სკრიპტი წერია

### Input (შენატანი)
- `mouse_x`, `mouse_y`  
მაუსის ისრის მიმდინარე პოზიცია

- `btn(key)`  
აბრუნებს `True`-ს თუ `key` დაჭერილია, სხვა შემთხვევაში აბრუნებს `False`-s ([საიდენტიფიკაციო კოდების განმარტების სია](https://github.com/kitao/pyxel/blob/master/pyxel/constants.py))

- `btnp(key, [hold], [period])`  
აბრუნებს `True`-ს თუ `key` დაჭერილია კონკრეტულად მიმდინარე კადრის დროს, სხვა შემთხვევაში აბრუნებს `False`-s. როდესაც `hold` და `period` არის განსაზღვრული, `True` დაბრუნებული იქნება `period` კადრის ინტერვალში როცა `key` დაჭერილი იქნება უფრო მეტხანს, ვიდრე `hold`-ის კადრების დროს

- `btnr(key)`  
აბრუნებს `True`-ს თუ `key` აშვებულია კონკრეტულად მიმდინარე კადრის დროს, სხვა შემთხვევაში აბრუნებს `False`-s

### გრაფიკა

- `image(img, [system])`  
გამოსახულების ბანკზე ოპერირება `img`(0-2) (იხილეთ გამოსახულების კლასი). თუ `system` არის `True`, სისტემის გამოსახულების ბანკი 3 ხდება ხელმისაწვდომი  
მაგალითად: `pyxel.image(0).load(0, 0, 'title.png')`

- `clip(x1, y1, x2, y2)`  
დასახატი არეალის დაყენება სიზუსტით (`x1`, `y1`)-(`x2`, `y2`). დასახატი არეალის საწყის მდგომარეობაში დაბრუნება ხდება `clip()`-ით

- `pal(col1, col2)`  
ხატვისას შეცვლა `col1` ფერის `col2`-ით. `pal()`, რომ პალიტრა დაბრუნდეს საწყის მდგომარეობაში

- `cls(col)`  
ეკრანის გასუფთავება `col` ფერით

- `pix(x, y, col)`  
პიქსელის დახატვა `col` ფერით (`x`, `y`) კოორდინატზე

- `line(x1, y1, x2, y2, col)`  
ხაზის დახატვა `col` ფერით (`x1`, `y1`)-დან (`x2`, `y2`) კოორდინატამდე

- `rect(x1, y1, x2, y2, col)`  
ოთხკუთხედის დახატვა `col` ფერით (`x1`, `y1`)-დან (`x2`, `y2`) კოორდინატამდე

- `rectb(x1, y1, x2, y2, col)`  
ოთხკუთხედის გარეხაზის (კიდეების) დახატვა `col` ფერით (`x1`, `y1`)-დან (`x2`, `y2`) კოორდინატამდე

- `circ(x, y, r, col)`  
წრის დახატვა `r` რადიუსით და `col` ფერით (`x`, `y`) კოორდინატზე

- `circb(x, y, r, col)`  
წრეწირის დახატვა `r` რადიუსით და `col` ფერით (`x`, `y`) კოორდინატზე

- `blt(x, y, img, sx, sy, w, h, [colkey])`  
(`w`, `h`) ზომის რეგიონის დაკოპირება (`sx`, `sy`)-ის გამოსახულების ბანკიდან `img`(0-2) (`x`, `y`) კოორდინატამდე. თუ `w` და/ან `h` მნიშვნელობები უარყოფითია, მაშინ გამოსახულება საწინააღმდეგოდ შეიცვლება ჰორიზონტალურად და/ან ვერტიკალურად. თუ `colkey` განსაზღვრულია, ხდება გამჭირვალე  ფერად აღქმა

- `text(x, y, s, col)`  
`s` სტრიქონის დახატვა `col` ფერით (`x`, `y`) კოორდინატზე

### აუდიო

- `sound(snd)`  
ხმოვან ბანკზე ოპერირება `snd`(0-63) (იხილეთ ხმის კლასი)  
მაგალითად: `pyxel.sound(0).speed = 60`

- `play(ch, snd, loop=False)`  
`snd`(0-63) ხმის ბანკის ჩართვა `ch`(0-3) არხზე. ჩართვა განხორციელდება თანმიმდევრობით თუ `snd` რეალიზებულია როგორც სია

- `stop(ch)`  
`ch`(0-3) არხის აუდიო მასალის ხელახლა ჩართვის შეჩერება

### გამოსახულების კლასი

- `width`, `height`  
გამოსახულების სიგანე და სიმაღლე

- `data`  
გამოსახულების მონაცემები (NumPy ბიბლიოთეკის მეშვეობით წარმოქმნილი რიგი)

- `set(x, y, data)`  
გამოსახულების დაყენება სტრიქონების სიად (`x`, `y`) კოორდინატზე  
მაგალითად: `pyxel.image(0).set(10, 10, ['1234', '5678', '9abc', 'defg'])`

- `load(x, y, filename)`  
png გაფართოების გამოსახულების წაკითხვა იმ დირექტორიდან, სადაც გაშვების სკრიპტი წერია (`x`, `y`) კოორდინატზე

- `copy(x, y, img, sx, sy, width, height)`  
(`width`, `height`) ზომის რეგიონის დაკოპირება (`sx`, `sy`)-ის გამოსახულების ბანკიდან `img`(0-2) (`x`, `y`) კოორდინატამდე

### ხმის კლასი

- `note`  
ნოტების სია (0-127) (33 = 'A2' = 440Hz)

- `tone`  
ტონების სია (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volume`  
მოცულობის სია (0-7)

- `effect`  
ეფექტის სია (0:None / 1:Slide / 2:Vibrato / 3:FadeOut)

- `speed`  
ერთი ნოტის სიგრძე (120 = 1 წამი თითო ტონზე)

- `set(note, tone, volume, effect, speed)`  
ნოტის, ტონის, მოცულობის და ეფექტის განსაზღვრა სტრინგით. თუ ტონი, მოცულობა და ეფექტის სიგრძე უფრო მოკლეა ნოტთან შედარებით, მაშინ გამეორება მოხდება დასაწყისიდან

- `set_note(note)`  
ნოტის განსაზღვრა მომდევნო სტრინგების ელემენტებით 'CDEFGAB'+'#-'+'0123' ან 'R'. ლათინური ასოს ინიციალების სენსიტიურობას და გამოტოვებებს ყურადღება არ ექცევა  
მაგალითად: `pyxel.sound(0).set_note('G2B-2D3R RF3F3F3')`

- `set_tone(tone)`  
ტონის განსაზღვრა მომდევნო სტრინგის ელემენტებით 'TSPN'. ლათინური ასოს ინიციალების სენსიტიურობას და გამოტოვებებს ყურადღება არ ექცევა  
მაგალითად: `pyxel.sound(0).set_tone('TTSS PPPN')`

- `set_volume(volume)`  
მოცულობის განსაზღვრა მომდევნო სტრინგის ელემენტებით '01234567'. ლათინური ასოს ინიციალების სენსიტიურობას და გამოტოვებებს ყურადღება არ ექცევა  
მაგალითად: `pyxel.sound(0).set_volume('7777 7531')`

- `set_effect(effect)`  
ეფექტის განსაზღვრა მომდევნო სტრინგის ელემენტებით 'NSVF'. ლათინური ასოს ინიციალების სენსიტიურობას და გამოტოვებებს ყურადღება არ ექცევა  
მაგალითად: `pyxel.sound(0).set_effect('NFNF NVVS')`

## დამატებითი ინფორმაცია

- [Pyxel Wiki](https://github.com/kitao/pyxel/wiki)

## ლიცენზია

Pyxel-ი ექვემდებარება [MIT ლიცენზია](http://en.wikipedia.org/wiki/MIT_License)-ს. ის შეიძლება ხელახლა გამოყენებული იქნას მესაკუთრეობრივი პროგრამული უზრუნველყოფის მიერ იმ შემთხვევაში, თუ ყველა არსებული ლიცენზირებული პროგრამული უზრუნველყოფის ასლი აღჭურვილი იქნება MIT ლიცენზიის პირობების ასლით და ამავდროულად საავტორო უფლებები იქნება დაცული.
