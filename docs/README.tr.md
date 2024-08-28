# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](https://github.com/kitao/pyxel/blob/main//README.md) | [中文](https://github.com/kitao/pyxel/blob/main//docs/README.cn.md) | [Deutsch](https://github.com/kitao/pyxel/blob/main//docs/README.de.md) | [Español](https://github.com/kitao/pyxel/blob/main//docs/README.es.md) | [Français](https://github.com/kitao/pyxel/blob/main//docs/README.fr.md) | [Italiano](https://github.com/kitao/pyxel/blob/main//docs/README.it.md) | [日本語](https://github.com/kitao/pyxel/blob/main//docs/README.ja.md) | [한국어](https://github.com/kitao/pyxel/blob/main//docs/README.ko.md) | [Português](https://github.com/kitao/pyxel/blob/main//docs/README.pt.md) | [Русский](https://github.com/kitao/pyxel/blob/main//docs/README.ru.md) | [Türkçe](https://github.com/kitao/pyxel/blob/main//docs/README.tr.md) | [Українська](https://github.com/kitao/pyxel/blob/main//docs/README.uk.md) ]

**Pyxel** Python için bir retro oyun motorudur.

Retro oyun konsollarından esinlenerek basitleştirilmiş özelliklere sahip olması sayesinde, aynı anda yalnızca 16 renk görüntülenebildiği ve yalnızca 4 ses çalınabildiği için, pixel art tarzı oyunlar yapmanın keyfini doyasıya çıkarabilirsiniz.

<img src="images/pyxel_message.png" width="480">

Pyxel'in geliştirilmesinin motivasyonu kullanıcıların geri bildirimleridir. Lütfen Pyxel'e GitHub'da bir yıldız verin!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">
<img src="images/01_hello_pyxel.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">
<img src="images/03_draw_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">
<img src="images/04_sound_api.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Pyxel, [PICO-8](https://www.lexaloffle.com/pico-8.php) ve [TIC-80](https://tic80.com/) gibi retro oyun motorlarından esinlenerek tasarlanmıştır.

Pyxel tamamen ücretsiz ve açık kaynaklıdır. Haydi, Pyxel ile birlikte retro bir oyun yapalım!

## Özellikler

- Windows, Mac, Linux ve Web üzerinde çalışır
- Python ile programlandı
- 16 renk paleti
- 256x256 boyutlarında 3 resim seti
- 256x256 boyutlarında 8 tileset
- 64 adet tanımlanabilir ses ile 4 kanal
- Dilediğiniz sesleri birleştirebilen 8 müzik
- Klavye, fare ve gamepad girişi
- Resim ve ses düzenleyici

### Renk Paleti

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Nasıl Kurulur

### Windows

[Python3](https://www.python.org/) (sürüm 3.8 veya daha üstü) kurduktan sonra, aşağıdaki komutu çalıştırın:

```sh
pip install -U pyxel
```

Python'u resmi yükleyici ile kurarsanız, `pyxel` komutunu etkinleştirmek için `Add Python 3.x to PATH` seçeneğini işaretleyin.

### Mac

[Homebrew](https://brew.sh/) kurduktan sonra, aşağıdaki komutları çalıştırın:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Pyxel'i kurduktan sonra sürümü güncellemek için pipx upgrade pyxel komutunu çalıştırın.

### Linux

SDL2 paketini (`libsdl2-dev` Ubuntu için), [Python3](https://www.python.org/) (sürüm 3.8 veya daha üstü) ve `python3-pip` kurduktan sonra, aşağıdaki komutu çalıştırın:

```sh
sudo pip3 install -U pyxel
```

Yukarıdaki komut çalışmazsa, [Makefile](https://github.com/kitao/pyxel/blob/main//Makefile) talimatlarına göre kendiniz derlemeyi deneyin.

### Web

Pyxel'in web versiyonu Python veya Pyxel kurulumu gerektirmez ve desteklenen web tarayıcıları ile hem PC'lerde hem de akıllı telefon ve tabletlerde çalışır.

Özel talimatlar için lütfen [bu sayfaya](https://github.com/kitao/pyxel/wiki/How-To-Use-Pyxel-Web). başvurun.

### Pyxel Örneklerini Deneyin

Pyxel'i kurduktan sonra, aşağıdaki komutla Pyxel örnekleri mevcut dizine kopyalanacaktır:

```sh
pyxel copy_examples
```

Kopyalanacak örnekler aşağıdaki gibidir:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>En basit uygulama</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Code</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Pyxel kaynak dosyası ile zıplama oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Code</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Çizim API'lerinin gösterimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Code</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Ses API'lerinin gösterimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Code</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Renk paleti listesi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Code</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Fare tıklama oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Code</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>BGM ile yılan oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Code</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Üçgen çizim API'lerinin gösterimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Code</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Ekran geçişli bir shoot'em up oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Code</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Harita ile yan kaydırmalı platform oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Code</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Image sınıfı ile ekran dışı renderlama</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Code</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Perlin gürültü animasyonu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Code</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Bitmap font çizimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Code</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Ses genişletme özelliklerini kullanan synthesizer</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Code</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Tile Map Dosyası (.tmx) yükleme ve çizme</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Code</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Görüntü döndürme ve ölçekleme</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Code</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>Flip fonksiyonu ile animasyon (sadece web dışı platformlar)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Code</a></td>
</tr>
<tr>
<td>30SecondsOfDaylight.pyxapp</td>
<td>1. Pyxel Jam kazanan oyunu <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30SecondsOfDaylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Code</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Arcade top fiziği oyunu <a href="https://x.com/helpcomputer0">Adam</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/helpcomputer/megaball">Code</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Arka plan müzik jeneratörü <a href="https://x.com/frenchbread1222">frenchbread</a></td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Code</a></td>
</tr>
</table>

Bir örnek aşağıdaki komutlarla çalıştırılabilir:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30SecondsOfDaylight.pyxapp
```

## Nasıl Kullanılır

### Pyxel Uygulaması Oluşturma

Python betiğinizde Pyxel modülünü içe aktardıktan sonra, ilk olarak `init` fonksiyonu ile pencere boyutunu belirtin ve ardından `run` fonksiyonu ile Pyxel uygulamasını başlatın.

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

`run` fonksiyonunun argümanları, her kareyi güncellemek için `update` fonksiyonu ve gerektiğinde ekranı çizmek için `draw` fonksiyonudur.

Gerçek bir uygulamada, pyxel kodlarını aşağıdaki gibi bir sınıfa sarmanız önerilir:

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
        pyxel.rect(self.x, 0, 8, 8, 9)

App()
```

Basit grafikler oluştururken, animasyon olmadan kodu daha kısa yapmak için `show` fonksiyonu kullanılabilir.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Pyxel Uygulamasını Çalıştırma

Oluşturulan bir Python betiği `python` komutu kullanılarak çalıştırılabilir:

```sh
python PYTHON_SCRIPT_FILE
```

Ayrıca `pyxel run` komutu ile de çalıştırılabilir:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Ek olarak, `pyxel watch` komutu, belirtilen bir dizindeki değişikliklerin izlenmesini sağlar ve değişiklikler tespit edildiğinde programı otomatik olarak yeniden çalıştırır:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Dizin izleme `Ctrl(Command)+C` tuşlarına basılarak durdurulabilir.

### Özel Komutlar

Pyxel uygulaması çalışırken aşağıdaki özel komutlar kullanılabilir:

- `Esc`<br>
  Uygulamadan çıkış yapar
- `Alt(Option)+1`<br>
  Ekran görüntüsünü masaüstüne kaydeder
- `Alt(Option)+2`<br>
  Ekran videosu kaydının başlangıç zamanını sıfırlar
- `Alt(Option)+3`<br>
  Ekran videosunu masaüstüne kaydeder (en fazla 10 saniye)
- `Alt(Option)+9`<br>
  Ekran modları arasında geçiş yapar (Crisp/Smooth/Retro)
- `Alt(Option)+0`<br>
  Performans monitörünü açıp kapatır (fps, güncelleme süresi ve çizim süresi)
- `Alt(Option)+Enter`<br>
  Tam ekran modunu açıp kapatır
- `Shift+Alt(Option)+1/2/3`<br>
  İlgili görüntü setini masaüstüne kaydeder
- `Shift+Alt(Option)+0`<br>
  Mevcut renk paletini masaüstüne kaydeder

### Kaynaklar Nasıl Oluşturulur

Pyxel Editör, Pyxel uygulamasında kullanılan görüntü ve sesleri oluşturabilir.

Editörü başlatmak için şu komut kullanılır:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Belirtilen Pyxel kaynak dosyası (.pyxres) mevcutsa, dosya yüklenir; mevcut değilse belirtilen isimle yeni bir dosya oluşturulur (`my_resource.pyxres` adıyla).

Editör başlatıldıktan sonra, başka bir kaynak dosyasını sürükleyip bırakarak dosya değiştirilebilir.

Oluşturulan kaynak dosyası `load` fonksiyonu ile yüklenebilir.

Pyxel Editör'ün aşağıdaki düzenleme modları bulunmaktadır.

**Image Editor**

Görüntü setlerini düzenleme modu.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Görüntüyü mevcut olarak seçilmiş görüntü setine yüklemek için Görüntü Düzenleyici'ye bir görüntü dosyasını (PNG/GIF/JPEG) sürükleyip bırakın.

**Tilemap Editor**

Görüntü setlerindeki görüntülerin bir tile deseninde düzenlendiği tilemap'leri düzenleme modu.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Bir TMX dosyasını (Tiled Map Dosyası) Tilemap Editöre sürükleyip bırakın. Bu, seçili döşeme harita numarasına karşılık gelen çizim sırasındaki katmanını yükler.

**Sound Editor**

Sesleri düzenleme modu.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Music Editor**

Seslerin çalma sırasına göre düzenlendiği müzikleri düzenleme modu.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Diğer Kaynak Oluşturma Yöntemleri

Pyxel görüntüleri ve tilemap'leri aşağıdaki yöntemlerle de oluşturulabilir:

- `Image.set` veya `Tilemap.set` fonksiyonu ile bir dizi string'den bir görüntü oluşturulabilir
- `Image.load` fonksiyonu ile bir görüntü dosyası (PNG/GIF/JPEG) Pyxel paletine yüklenebilir

Pyxel sesleri aşağıdaki yöntemle de oluşturulabilir:

- `Sound.set` veya `Music.set` fonksiyonu ile string'lerden bir ses oluşturulabilir

Bu fonksiyonların kullanımı için API referansına başvurun.

### Uygulamaları Nasıl Dağıtılır

Pyxel, platformlar arası çalışan özel bir uygulama dağıtım dosya formatını (Pyxel uygulama dosyası) destekler.

Pyxel uygulama dosyasını (.pyxapp) `pyxel package` komutu ile oluşturun:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Uygulamanın kaynak veya ek modülleri içermesi gerekiyorsa, bunları uygulama dizinine yerleştirin.

Oluşturulan uygulama dosyası `pyxel play` komutu ile çalıştırılabilir:

```sh
pyxel play PYXEL_APP_FILE
```

Pyxel uygulama dosyası ayrıca `pyxel app2exe` veya `pyxel app2html` komutları ile bir yürütülebilir dosyaya veya HTML dosyasına dönüştürülebilir.

## API Referansı

### Sistem

- `width`, `height`<br>
  Ekranın genişliği ve yüksekliği

- `frame_count`<br>
  Geçen kare sayısı

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Pyxel uygulamasını ekran boyutu (`width`, `height`) ile başlatır. İsteğe bağlı olarak aşağıdaki seçenekler belirtilebilir: pencere başlığı `title`, kare hızı `fps`, uygulamadan çıkış için kullanılacak tuş `quit_key`, ekran görüntüsü ölçeği `display_scale`, yakalama ölçeği `capture_scale`, ve ekran videosu maksimum kayıt süresi `capture_sec`.<br>
  Örnek: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Pyxel uygulamasını başlatır ve her kare için `update` fonksiyonunu, ekrana çizim için `draw` fonksiyonunu çağırır.

- `show()`<br>
  Ekranı gösterir ve `Esc` tuşuna basılana kadar bekler.

- `flip()`<br>
  Ekranı bir kare yeniler. Uygulama, `Esc` tuşuna basıldığında çıkar. Bu fonksiyon web sürümünde çalışmaz.

- `quit()`<br>
  Pyxel uygulamasını kapatır.

### Kaynaklar

- `load(filename, [excl_images], [excl_tilemaps], [excl_sounds], [excl_musics])`<br>
  Kaynak dosyasını (.pyxres) yükle. Bir seçenek `True` ise, kaynak yüklenmeyecek. Aynı konumda kaynak dosyası ile aynı ada sahip bir palet dosyası (.pyxpal) varsa, palet ekran renkleri değiştirilecek. Palet dosyası, görüntü renklerinin onaltılık girişleridir (örneğin, `1100FF`), yeni satırlarla ayrılmıştır. Palet dosyası ayrıca, Pyxel Editor'de görüntülenen renkleri değiştirmek için kullanılabilir.

### Giriş İşlemleri

- `mouse_x`, `mouse_y`<br>
  Fare imlecinin mevcut konumu

- `mouse_wheel`<br>
  Fare tekerleğinin mevcut değeri

- `btn(key)`<br>
  `key` tuşu basılıysa `True` döndürür, aksi takdirde `False`. ([Tuş tanımı listesi](https://github.com/kitao/pyxel/blob/main//python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  `key` tuşu o karede basılıysa `True` döndürür. `hold` ve `repeat` belirtilmişse, `key` tuşu `hold` kare süresinden uzun süre basılı tutulduğunda `repeat` kare aralığında `True` döner.

- `btnr(key)`<br>
  `key` tuşu o karede bırakıldıysa `True` döndürür.

- `mouse(visible)`<br>
  Eğer `visible` `True` ise fare imleci gösterilir. `False` ise gizlenir. Fare imleci gösterilmiyor olsa bile konumu güncellenir.

### Grafikler

- `colors`<br>
  Palet gösterim renklerinin listesi. Gösterim rengi 24-bit sayısal bir değerle belirtilir. Python listelerini doğrudan atamak ve almak için `colors.from_list` ve `colors.to_list` kullanın.<br>
  Örneğin: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Görüntü setlerinin (0-2) listesi. (Resim sınıfına bakınız)<br>
  Örneğin: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Tile haritalarının (0-7) listesi. (Tilemap sınıfına bakınız)

- `clip(x, y, w, h)`<br>
  Ekranın çizim alanını (`x`, `y`) konumundan `w` genişlik ve `h` yükseklik olarak ayarlar. `clip()` ile çizim alanını tam ekran olarak sıfırlar.

- `camera(x, y)`<br>
  Ekranın sol üst köşe koordinatlarını (`x`, `y`) değiştirir. `camera()` ile sol üst köşe koordinatlarını (`0`, `0`) olarak sıfırlar.

- `pal(col1, col2)`<br>
  Çizim sırasında `col1` rengini `col2` ile değiştirir. Başlangıç paletine dönmek için `pal()` kullanılır.

- `dither(alpha)`<br>
  Çizim sırasında saydamlık uygular. 0,0 ile 1,0 arasında `alpha` değeri ayarlanır, 0,0 saydam ve 1,0 opak anlamına gelir.

- `cls(col)`<br>
  Ekranı `col` renk ile temizler.

- `pget(x, y)`<br>
  (`x`, `y`) konumundaki pikselin rengini alır.

- `pset(x, y, col)`<br>
  (`x`, `y`) konumuna `col` renginde bir piksel çizer.

- `line(x1, y1, x2, y2, col)`<br>
  (`x1`, `y1`) noktasından (`x2`, `y2`) noktasına `col` renkli bir çizgi çizer.

- `rect(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından başlayarak `w` genişlik ve `h` yükseklikte `col` renkli bir dikdörtgen çizer.

- `rectb(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından başlayarak `w` genişlik ve `h` yükseklikte `col` renkli dikdörtgenin sınırlarını çizer.

- `circ(x, y, r, col)`<br>
  (`x`, `y`) merkezinden başlayarak yarıçapı `r` ve `col` renkli bir daire çizer.

- `circb(x, y, r, col)`<br>
  (`x`, `y`) merkezinden başlayarak yarıçapı `r` ve `col` renkli dairenin sınırlarını çizer.

- `elli(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından başlayarak `w` genişlik ve `h` yükseklikte `col` renkli bir elips çizer.

- `ellib(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından başlayarak `w` genişlik ve `h` yükseklikte `col` renkli elipsin sınırlarını çizer.

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) noktalarından geçen `col` renkli bir üçgen çizer.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) noktalarından geçen `col` renkli üçgenin sınırlarını çizer.

- `fill(x, y, col)`<br>
  (`x`, `y`) noktasından başlayarak aynı renk ile bağlantılı alanı `col` renkle doldurur.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  `img` (0-2) resim setinden (`u`, `v`) konumundan başlayarak boyutu (`w`, `h`) olan bölümü (`x`, `y`) konumuna kopyalar. `w` ve/veya `h` için negatif bir değer ayarlanırsa, yatay ve/veya dikey olarak ters çevrilir. `colkey` belirtilmişse, saydam renk olarak işlenir. Eğer `rotate`(derece cinsinden), `scale`(1.0=%100) veya her ikisi de belirtilirse, ilgili dönüşüm uygulanacaktır.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  `tm` (0-7) tile haritasından (`u`, `v`) konumundan başlayarak boyutu (`w`, `h`) olan bölümü (`x`, `y`) konumuna kopyalar. `w` ve/veya `h` için negatif bir değer ayarlanırsa, yatay ve/veya dikey olarak ters çevrilir. `colkey` belirtilmişse, saydam renk olarak işlenir. Eğer `rotate`(derece cinsinden), `scale`(1.0=%100) veya her ikisi de belirtilirse, ilgili dönüşüm uygulanacaktır. Bir tile'ın boyutu 8x8 pikseldir ve tile haritasında `(tile_x, tile_y)` olarak saklanır.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  (`x`, `y`) noktasından başlayarak `s` metnini `col` renk ile çizer.

  ### Ses

- `sounds`<br>
  Seslerin (0-63) listesi. (Sound sınıfına bakınız)<br>
  Örneğin: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Müziklerin (0-7) listesi. (Music sınıfına bakınız)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  `snd`(0-63) sesini `ch`(0-3) kanalında çalar. Eğer `snd` bir liste ise, sırayla çalınır. Çalma başlangıç pozisyonu `tick` ile belirtilebilir (1 tick = 1/120 saniye). Eğer `loop` için `True` belirtilmişse, müzik döngü şeklinde çalar. Çalma bitiminde önceki sesi devam ettirmek için `resume` için `True` belirtin.

- `playm(msc, [tick], [loop])`<br>
  `msc`(0-7) müziğini çalar. Çalma başlangıç pozisyonu `tick` ile belirtilebilir (1 tick = 1/120 saniye). Eğer `loop` için `True` belirtilmişse, müzik döngü şeklinde çalar.

- `stop([ch])`<br>
  Belirtilen kanalda `ch`(0-3) müziğin çalmasını durdurur. Tüm kanalları durdurmak için `stop()` kullanılır.

- `play_pos(ch)`<br>
  `ch`(0-3) kanalının ses çalma pozisyonunu `(ses no, nota no)` olarak alır. Çalma durduğunda `None` döner.

### Matematik

- `ceil(x)`<br>
  `x` değerinden büyük veya eşit olan en küçük tamsayıyı döndürür.

- `floor(x)`<br>
  `x` değerinden küçük veya eşit olan en büyük tamsayıyı döndürür.

- `sgn(x)`<br>
  `x` pozitif ise 1, sıfır ise 0, negatif ise -1 döndürür.

- `sqrt(x)`<br>
  `x`'in karekökünü döndürür.

- `sin(deg)`<br>
  `deg` derecesinin sinüsünü döndürür.

- `cos(deg)`<br>
  `deg` derecesinin kosinüsünü döndürür.

- `atan2(y, x)`<br>
  `y`/`x` için arktanjantı derece cinsinden döndürür.

- `rseed(seed)`<br>
  Rastgele sayı üretecinin tohumunu ayarlar.

- `rndi(a, b)`<br>
  `a` ve `b` aralığında rastgele bir tamsayı döndürür (a ve b dahil).

- `rndf(a, b)`<br>
  `a` ve `b` aralığında rastgele bir ondalıklı sayı döndürür (a ve b dahil).

- `nseed(seed)`<br>
  Perlin gürültüsünün tohumunu ayarlar.

- `noise(x, [y], [z])`<br>
  Belirtilen koordinatlar için Perlin gürültüsü değerini döndürür.

### Image Sınıfı

- `width`, `height` <br>
  Resmin genişliği ve yüksekliği

- `set(x, y, data)`<br>
  (`x`, `y`) konumundaki resmi bir dizi string ile ayarlar.
  Örneğin: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  Resim dosyasını (PNG/GIF/JPEG) (`x`, `y`) konumuna yükler.

- `pget(x, y)`<br>
  (`x`, `y`) konumundaki piksel rengini alır.

- `pset(x, y, col)`<br>
  (`x`, `y`) konumuna `col` renkli bir piksel çizer.

### Tilemap Sınıfı

- `width`, `height`<br>
  Tilemap'in genişliği ve yüksekliği

- `imgsrc`<br>
  Tilemap tarafından referans alınan resim seti (0-2)

- `set(x, y, data)`<br>
  (`x`, `y`) konumundaki tilemap'i bir dizi string ile ayarlar.
  Örneğin: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  `layer`(0-) sırasında çizim sırasındaki katmanı belirterek (`x`, `y`) konumundaki TMX dosyasından (Tiled Harita Dosyası) katmanı yükler.

- `pget(x, y)`<br>
  (`x`, `y`) konumundaki tile'ı alır. Bir tile, `(tile_x, tile_y)` tuple'ıdır.

- `pset(x, y, tile)`<br>
  (`x`, `y`) konumuna bir `tile` çizer. Bir tile, `(tile_x, tile_y)` tuple'ıdır.

### Ses Sınıfı

- `notes`<br>
  Notaların listesi (0-127). Numara ne kadar yüksekse, sesin tonu o kadar yüksek olur ve 33'te 'A2' (440Hz) olur. Geri kalanı -1'dir.

- `tones`<br>
  Tonların listesi (0:Dreieck / 1:Quadrat / 2:Puls / 3:Rauschen)

- `volumes`<br>
  Ses düzeylerinin listesi (0-7)

- `effects`<br>
  Efektlerin listesi (0:Yok / 1:Kaydırma / 2:Titreme / 3:Sönme / 4:Yarı Sönme / 5:Dörtte Bir Sönme)

- `speed`<br>
  Çalma hızı. 1 en hızlısıdır, sayı ne kadar büyükse çalma hızı o kadar yavaş olur. 120'de bir nota uzunluğu 1 saniyeye eşittir.

- `set(notes, tones, volumes, effects, speed)`<br>
  Notaları, tonları, ses düzeylerini ve efektleri bir dize ile ayarlar. Eğer tonlar, ses düzeyleri ve efektler notalardan daha kısa ise, baştan tekrarlanır.

- `set_notes(notes)`<br>
  'CDEFGAB'+'#-'+'01234' veya 'R' karakterlerinden oluşan bir dize ile notaları ayarlar. Büyük-küçük harf duyarsızdır ve boşluklar yok sayılır.
  Örneğin: `pyxel.sounds[0].set_notes("G2B-2D3R RF3F3F3")`

- `set_tones(tones)`<br>
  'TSPN' karakterlerinden oluşan bir dize ile tonları ayarlar. Büyük-küçük harf duyarsızdır ve boşluklar yok sayılır.
  Örneğin: `pyxel.sounds[0].set_tones("TTSS PPPN")`

- `set_volumes(volumes)`<br>
  '01234567' karakterlerinden oluşan bir dize ile ses düzeylerini ayarlar. Büyük-küçük harf duyarsızdır ve boşluklar yok sayılır.
  Örneğin: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  'NSVFHQ' karakterlerinden oluşan bir dize ile efektleri ayarlar. Büyük-küçük harf duyarsızdır ve boşluklar yok sayılır.
  Örneğin: `pyxel.sounds[0].set_effects("NFNF NVVS")`

### Müzik Sınıfı

- `seqs`<br>
  Kanal sayısıyla birlikte seslerin (0-63) iki boyutlu listesi

- `set(seq0, seq1, seq2, ...)`<br>
  Kanalların ses listelerini ayarlar. Eğer boş bir liste belirtilirse, o kanal çalma için kullanılmaz.<br>
  Örneğin: `pyxel.musics[0].set([0, 1], [], [3])`

### İleri Düzey API'lar

Pyxel, kullanıcıları karıştırabilecek veya kullanmak için özelleşmiş bilgi gerektirebilecek "ileri düzey API'ları" içerir. Bu API'lar bu referansta belirtilmemiştir.

Eğer becerilerinize güveniyorsanız, [bu bağlantıya](https://github.com/kitao/pyxel/blob/main//python/pyxel/__init__.pyi) bakarak muhteşem çalışmalar oluşturabilirsiniz!

## Nasıl Katkıda Bulunulur

### Sorun Bildirme

Hata raporları ve özellik/geliştirme isteklerini göndermek için [Sorun İzleyici](https://github.com/kitao/pyxel/issues) kullanın. Yeni bir sorun bildirmeden önce, benzer bir açık sorun olmadığından emin olun.

### Manuel Test

Kodları manuel olarak test eden ve [Sorun İzleyici](https://github.com/kitao/pyxel/issues) üzerinden hataları raporlayan veya geliştirme önerileri sunan herkes çok memnuniyetle karşılanır!

### Pull Request Gönderme

Yama/düzeltmeler, pull request (PR) olarak kabul edilir. Pull request'inizle ilgili sorunun Sorun İzleyici'de açık olduğundan emin olun.

Gönderilen pull request, [MIT Lisansı](https://github.com/kitao/pyxel/blob/main//LICENSE) altında yayımlamayı kabul etmiş sayılır.

## Diğer Bilgiler

- [SORU-CEVAP](https://github.com/kitao/pyxel/wiki/Pyxel-Q&A)
- [Kullanıcı Örnekleri](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Geliştirici X hesabı](https://x.com/kitao)

## Lisans

Pyxel, [MIT Lisansı](https://github.com/kitao/pyxel/blob/main//LICENSE) altındadır. Tüm kopyalarında veya önemli bölümlerinde, MIT Lisansı'nın şartlarının bir kopyası ve ayrıca bir telif hakkı bildirimi bulunması koşuluyla, bu yazılım özel yazılımlar içinde yeniden kullanılabilir.

## Sponsorlar Arıyoruz

Pyxel, GitHub Sponsorlarında sponsor arıyor. Pyxel'in sürekli bakımı ve özellik eklemeleri için sponsorluk düşünün. Sponsorlar, Pyxel hakkında danışmanlık hizmeti gibi avantajlardan yararlanabilirler. Detaylar için [buraya](https://github.com/sponsors/kitao) bakınız.
