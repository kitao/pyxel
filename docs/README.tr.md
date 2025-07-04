# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel**, Python için bir retro oyun motorudur.

Özellikler, yalnızca 16 renk desteği ve 4 ses kanalıyla retro oyun konsollarından ilham alınarak tasarlanmıştır, böylece piksel sanat tarzı oyunlar yapmayı kolayca keyifle yaşayabilirsiniz.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Pyxel'in geliştirilmesi, kullanıcı geri bildirimleriyle yönlendirilmektedir. Lütfen GitHub'da Pyxel'e bir yıldız verin!

<p>
<a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_music_editor.gif" width="320">
</a>
</p>

Pyxel'in spesifikasyonları ve API'leri, [PICO-8](https://www.lexaloffle.com/pico-8.php) ve [TIC-80](https://tic80.com/) tarafından ilham alınarak hazırlanmıştır.

Pyxel, [MIT Lisansı](../LICENSE) altında açık kaynaklıdır ve ücretsiz olarak kullanılabilir. Haydi, Pyxel ile retro oyun yapmaya başlayalım!

## Spesifikasyonlar

- Windows, Mac, Linux ve Web üzerinde çalışır
- Python ile programlama
- Özelleştirilebilir ekran boyutu
- 16 renk paleti
- 3 adet 256x256 boyutunda görüntü bankası
- 8 adet 256x256 boyutunda karo haritası
- 64 tanımlanabilir ses ile 4 kanal
- Herhangi bir sesi birleştirebilen 8 müzik parçası
- Klavye, fare ve gamepad girişi
- Görüntü ve ses düzenleme araçları
- Kullanıcı tarafından genişletilebilir renkler, kanallar ve bankalar

### Renk Paleti

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Nasıl Kurulur

### Windows

[Python3](https://www.python.org/) (3.8 veya daha yüksek sürüm) kurduktan sonra, aşağıdaki komutu çalıştırın:

```sh
pip install -U pyxel
```

Python'u resmi yükleyici ile kurarken, `Add Python 3.x to PATH` seçeneğini işaretlemeyi unutmayın, böylece `pyxel` komutunu etkinleştirmiş olursunuz.

### Mac

[Homebrew](https://brew.sh/) kurduktan sonra, aşağıdaki komutları çalıştırın:

```sh
brew install pipx
pipx ensurepath
pipx install pyxel
```

Kurulumdan sonra Pyxel'i güncellemek için `pipx upgrade pyxel` komutunu çalıştırın.

### Linux

SDL2 paketini (`libsdl2-dev` Ubuntu için), [Python3](https://www.python.org/) (3.8 veya daha yüksek sürüm) ve `python3-pip` kurduktan sonra, aşağıdaki komutu çalıştırın:

```sh
sudo pip3 install -U pyxel
```

Yukarıdaki komut çalışmazsa, Pyxel'i kaynak kodundan inşa etmeyi düşünün ve [Makefile](../Makefile) içindeki talimatları izleyin.

### Web

Pyxel'in web sürümü Python veya Pyxel kurulumu gerektirmeden çalışır ve desteklenen web tarayıcılarına sahip PC'ler, akıllı telefonlar ve tabletlerde çalışır.

Detaylı talimatlar için [bu sayfaya](pyxel-web-en.md) başvurun.

### Örnekleri Çalıştır

Pyxel'i kurduktan sonra, aşağıdaki komutla örnekleri geçerli dizine kopyalayabilirsiniz:

```sh
pyxel copy_examples
```

Aşağıdaki örnekler geçerli dizininize kopyalanacaktır:

<table>
<tr>
<td>01_hello_pyxel.py</td>
<td>En basit uygulama</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/01_hello_pyxel.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/01_hello_pyxel.py">Kod</a></td>
</tr>
<tr>
<td>02_jump_game.py</td>
<td>Pyxel kaynak dosyasıyla zıplama oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/02_jump_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/02_jump_game.py">Kod</a></td>
</tr>
<tr>
<td>03_draw_api.py</td>
<td>Çizim API'lerinin gösterimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/03_draw_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/03_draw_api.py">Kod</a></td>
</tr>
<tr>
<td>04_sound_api.py</td>
<td>Ses API'lerinin gösterimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/04_sound_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/04_sound_api.py">Kod</a></td>
</tr>
<tr>
<td>05_color_palette.py</td>
<td>Renk paletleri listesi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/05_color_palette.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/05_color_palette.py">Kod</a></td>
</tr>
<tr>
<td>06_click_game.py</td>
<td>Fare tıklama oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/06_click_game.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/06_click_game.py">Kod</a></td>
</tr>
<tr>
<td>07_snake.py</td>
<td>BGM'li yılan oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/07_snake.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/07_snake.py">Kod</a></td>
</tr>
<tr>
<td>08_triangle_api.py</td>
<td>Üçgen çizim API'lerinin gösterimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/08_triangle_api.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/08_triangle_api.py">Kod</a></td>
</tr>
<tr>
<td>09_shooter.py</td>
<td>Shoot'em up oyunu ile ekran geçişleri ve MML</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/09_shooter.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/09_shooter.py">Kod</a></td>
</tr>
<tr>
<td>10_platformer.py</td>
<td>Haritalı yan kaydırmalı platform oyunu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/10_platformer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/10_platformer.py">Kod</a></td>
</tr>
<tr>
<td>11_offscreen.py</td>
<td>Image sınıfıyla ekran dışı renderleme</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/11_offscreen.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/11_offscreen.py">Kod</a></td>
</tr>
<tr>
<td>12_perlin_noise.py</td>
<td>Perlin gürültüsü animasyonu</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/12_perlin_noise.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/12_perlin_noise.py">Kod</a></td>
</tr>
<tr>
<td>13_bitmap_font.py</td>
<td>Bitmap font çizimi</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/13_bitmap_font.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/13_bitmap_font.py">Kod</a></td>
</tr>
<tr>
<td>14_synthesizer.py</td>
<td>Ses genişletme özelliklerini kullanan sentezleyici</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/14_synthesizer.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/14_synthesizer.py">Kod</a></td>
</tr>
<tr>
<td>15_tiled_map_file.py</td>
<td>Tiled Map File (.tmx) yükleme ve çizme</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/15_tiled_map_file.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/15_tiled_map_file.py">Kod</a></td>
</tr>
<tr>
<td>16_transform.py</td>
<td>Görüntü döndürme ve ölçeklendirme</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/16_transform.html">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/16_transform.py">Kod</a></td>
</tr>
<tr>
<td>99_flip_animation.py</td>
<td>flip fonksiyonu ile animasyon (sadece web dışı platformlar için)</td>
<td><a href="https://github.com/kitao/pyxel/blob/main/docs/images/99_flip_animation.gif">Demo</a></td>
<td><a href="https://github.com/kitao/pyxel/blob/main/python/pyxel/examples/99_flip_animation.py">Kod</a></td>
</tr>
<tr>
<td>30sec_of_daylight.pyxapp</td>
<td>1. Pyxel Jam kazanan oyunu (<a href="https://x.com/helpcomputer0">Adam</a> tarafından)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/30sec_of_daylight.html">Demo</a></td>
<td><a href="https://github.com/kitao/30SecondsOfDaylight">Kod</a></td>
</tr>
<tr>
<td>megaball.pyxapp</td>
<td>Arcade top fizik oyunu (<a href="https://x.com/helpcomputer0">Adam</a> tarafından)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/megaball.html">Demo</a></td>
<td><a href="https://github.com/kitao/megaball">Kod</a></td>
</tr>
<tr>
<td>8bit-bgm-gen.pyxapp</td>
<td>Arka plan müziği oluşturucu (<a href="https://x.com/frenchbread1222">frenchbread</a> tarafından)</td>
<td><a href="https://kitao.github.io/pyxel/wasm/examples/8bit-bgm-gen.html">Demo</a></td>
<td><a href="https://github.com/shiromofufactory/8bit-bgm-generator">Kod</a></td>
</tr>
</table>

Örnekler aşağıdaki komutlarla çalıştırılabilir:

```sh
cd pyxel_examples
pyxel run 01_hello_pyxel.py
pyxel play 30sec_of_daylight.pyxapp
```

## Nasıl Kullanılır

### Uygulama Oluşturma

Python betiğinizde Pyxel modülünü içe aktarın, `init` fonksiyonu ile pencere boyutunu belirtin ve ardından `run` fonksiyonu ile Pyxel uygulamasını başlatın.

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

`run` fonksiyonunun argümanları, kare güncellemelerini işleyen `update` fonksiyonu ve ekran çizimini gerçekleştiren `draw` fonksiyonudur.

Gerçek bir uygulamada, Pyxel kodunu bir sınıf içine sarmak önerilir, aşağıdaki gibi:

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

Animasyon olmadan basit grafikler oluşturmak için, kodunuzu basitleştirmek için `show` fonksiyonunu kullanabilirsiniz.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Uygulamayı Çalıştırma

Oluşturulan bir betik `python` komutu ile çalıştırılabilir:

```sh
python PYTHON_SCRIPT_FILE
```

Ayrıca `pyxel run` komutu ile de çalıştırılabilir:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Ek olarak, `pyxel watch` komutu belirtilen bir dizindeki değişiklikleri izler ve değişiklik algılandığında programı otomatik olarak yeniden çalıştırır:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Dizin izlemeyi `Ctrl(Command)+C` tuşlarına basarak durdurabilirsiniz.

### Özel Tuş Operasyonları

Bir Pyxel uygulaması çalışırken, aşağıdaki özel tuş işlemleri gerçekleştirilebilir:

- `Esc`<br>
  Uygulamadan çık
- `Alt(Option)+1`<br>
  Ekran görüntüsünü masaüstüne kaydet
- `Alt(Option)+2`<br>
  Ekran kaydı videosunun başlangıç zamanını sıfırla
- `Alt(Option)+3`<br>
  Ekran kaydı videosunu masaüstüne kaydet (maksimum 10 saniye)
- `Alt(Option)+8` veya gamepad'de `A+B+X+Y+DL`<br>
  Ekran ölçeğini maksimum ve tam sayı arasında değiştir
- `Alt(Option)+9` veya gamepad'de `A+B+X+Y+DR`<br>
  Ekran modları arasında geçiş yap (Crisp/Smooth/Retro)
- `Alt(Option)+0` veya gamepad'de `A+B+X+Y+DU`<br>
  Performans monitörünü değiştir (FPS/`update` süresi/`draw` süresi)
- `Alt(Option)+Enter` veya gamepad'de `A+B+X+Y+DD`<br>
  Tam ekran modunu değiştir
- `Shift+Alt(Option)+1/2/3`<br>
  Görüntü bankası 0, 1 veya 2'yi masaüstüne kaydet
- `Shift+Alt(Option)+0`<br>
  Geçerli renk paletini masaüstüne kaydet

### Kaynakları Nasıl Oluşturulur

Pyxel Editor, bir Pyxel uygulamasında kullanılan resim ve sesleri oluşturabilir.

Pyxel Editor'ü aşağıdaki komut ile başlatabilirsiniz:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Belirtilen Pyxel kaynak dosyası (.pyxres) mevcutsa, yüklenecektir. Eğer mevcut değilse, belirtilen isimle yeni bir dosya oluşturulacaktır. Kaynak dosyası belirtilmezse, `my_resource.pyxres` adıyla yeni bir dosya oluşturulacaktır.

Pyxel Editor başlatıldıktan sonra, başka bir kaynak dosyasına geçmek için dosyayı Pyxel Editor'ün üzerine sürükleyip bırakabilirsiniz.

Oluşturulan kaynak dosyası, `load` fonksiyonu ile yüklenebilir.

Pyxel Editor aşağıdaki düzenleme modlarına sahiptir.

**Resim Editörü**

Her **resim bankasındaki** resmi düzenlemek için kullanılan mod.

<a href="https://kitao.github.io/pyxel/wasm/examples/image_editor.html">
<img src="images/image_editor.gif">
</a>

Resim editörüne bir resim dosyası (PNG/GIF/JPEG) sürükleyip bırakarak, resmi şu anda seçili olan resim bankasına yükleyebilirsiniz.

**Karo Haritası Editörü**

Resim bankalarındaki görüntüleri bir **karo haritası** içinde düzenlemek için kullanılan mod.

<a href="https://kitao.github.io/pyxel/wasm/examples/tilemap_editor.html">
<img src="images/tilemap_editor.gif">
</a>

Bir TMX dosyasını (Tiled Map File) karo haritası editörüne sürükleyip bırakarak, şu anda seçili olan karo haritasına katman 0'ı yükleyebilirsiniz.

**Ses Editörü**

Melodi ve **ses** efektlerinde kullanılan sesleri düzenlemek için mod.

<a href="https://kitao.github.io/pyxel/wasm/examples/sound_editor.html">
<img src="images/sound_editor.gif">
</a>

**Müzik Editörü**

Seslerin çalma sırasına göre dizildiği **müzikleri** düzenlemek için kullanılan mod.

<a href="https://kitao.github.io/pyxel/wasm/examples/music_editor.html">
<img src="images/music_editor.gif">
</a>

### Diğer Kaynak Oluşturma Yöntemleri

Pyxel resimleri ve karo haritaları aşağıdaki yöntemlerle de oluşturulabilir:

- `Image.set` fonksiyonu veya `Tilemap.set` fonksiyonu kullanarak bir dize listesinden resim oluşturun
- `Image.load` fonksiyonu ile Pyxel paletindeki bir resim dosyasını (PNG/GIF/JPEG) yükleyin

Pyxel sesleri de aşağıdaki yöntemle oluşturulabilir:

- `Sound.set` fonksiyonu veya `Music.set` fonksiyonu ile dizelerden ses oluşturun

Bu fonksiyonların kullanımı için API referansına bakın.

### Uygulamaları Nasıl Dağıtılır

Pyxel, platformlar arası çalışan özel bir uygulama dağıtım dosyası formatını (Pyxel uygulama dosyası) destekler.

Bir Pyxel uygulama dosyası (.pyxapp) `pyxel package` komutu kullanılarak oluşturulur:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Kaynakları veya ek modülleri dahil etmeniz gerekiyorsa, bunları uygulama dizinine yerleştirin.

Başlatma betiği içinde aşağıdaki formatla belirtilen meta veriler çalışma zamanında görüntülenebilir. `title` ve `author` dışında diğer alanlar isteğe bağlıdır.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

Oluşturulan uygulama dosyası `pyxel play` komutu kullanılarak çalıştırılabilir:

```sh
pyxel play PYXEL_APP_FILE
```

Bir Pyxel uygulama dosyası ayrıca `pyxel app2exe` veya `pyxel app2html` komutları kullanılarak çalıştırılabilir bir dosya veya HTML dosyasına dönüştürülebilir.

## API Başvurusu

### Sistem

- `width`, `height`<br>
  Ekranın genişliği ve yüksekliği

- `frame_count`<br>
  Geçen kare sayısı

- `init(width, height, [title], [fps], [quit_key], [display_scale], [capture_scale], [capture_sec])`<br>
  Pyxel uygulamasını ekran boyutlarıyla (`width`, `height`) başlatır. Şu seçenekler belirtilebilir: pencere başlığı `title`, kare hızı `fps`, uygulamayı kapatma tuşu `quit_key`, ekran ölçeği `display_scale`, ekran yakalama ölçeği `capture_scale` ve ekran yakalama videosunun maksimum kayıt süresi `capture_sec`.<br>
  Örnek: `pyxel.init(160, 120, title="My Pyxel App", fps=60, quit_key=pyxel.KEY_NONE, capture_scale=3, capture_sec=0)`

- `run(update, draw)`<br>
  Pyxel uygulamasını başlatır ve kare güncellemesi için `update` fonksiyonunu ve çizim için `draw` fonksiyonunu çağırır.

- `show()`<br>
  Ekranı gösterir ve `Esc` tuşuna basılana kadar bekler.

- `flip()`<br>
  Ekranı bir kare yeniler. Uygulama `Esc` tuşuna basıldığında kapanır. Bu fonksiyon web sürümünde çalışmaz.

- `quit()`<br>
  Pyxel uygulamasını kapatır.

### Kaynaklar

- `load(filename, [skip_images], [skip_tilemaps], [skip_sounds], [skip_musics])`<br>
  Kaynak dosyasını (.pyxres) yükler. Bir seçenek `True` olarak ayarlandığında, ilgili kaynak yüklemeden hariç tutulur. Eğer aynı konumda aynı ada sahip bir palet dosyası (.pyxpal) varsa, palet görüntü renkleri de güncellenir. Palet dosyası, görüntü renklerini 16'lık sayılar (örn. `1100ff`) ile satır satır içerir. Bu palet dosyası, Pyxel Editor'deki renkleri değiştirmek için de kullanılabilir.

- `user_data_dir(vendor_name, app_name)`<br>
  `vendor_name` ve `app_name` temel alınarak oluşturulan kullanıcı veri dizinini döndürür. Dizin mevcut değilse, otomatik olarak oluşturulur. Yüksek skorları, oyun ilerlemesini ve benzeri verileri saklamak için kullanılır.<br>
  Örnek: `print(pyxel.user_data_dir("Takashi Kitao", "Pyxel Shooter"))`

### Girdi

- `mouse_x`, `mouse_y`<br>
  Fare imlecinin mevcut konumu

- `mouse_wheel`<br>
  Fare tekerleğinin mevcut değeri

- `btn(key)`<br>
  `key` basılıysa `True`, basılı değilse `False` döndürür. ([TuĢ tanımları listesi](../python/pyxel/__init__.pyi))

- `btnp(key, [hold], [repeat])`<br>
  `key` o karede basılmışsa `True`, basılmamışsa `False` döndürür. Eğer `hold` ve `repeat` belirtilirse, `key` en az `hold` kare boyunca basılı tutulduktan sonra her `repeat` karede bir `True` döndürür.

- `btnr(key)`<br>
  `key` o karede serbest bırakılmışsa `True`, serbest bırakılmamışsa `False` döndürür.

- `mouse(visible)`<br>
  `visible` `True` ise fare imlecini gösterir, `False` ise gizler. İmleç gizlenmiş olsa bile konumu güncellenmeye devam eder.

### Grafikler

- `colors`<br>
  Palet görüntü renkleri listesi. Görüntü rengi 24 bitlik sayısal bir değerle belirtilir. Python listelerini doğrudan atamak ve almak için `colors.from_list` ve `colors.to_list` kullanılır.<br>
  Örnek: `old_colors = pyxel.colors.to_list(); pyxel.colors.from_list([0x111111, 0x222222, 0x333333]); pyxel.colors[15] = 0x112233`

- `images`<br>
  Görüntü bankalarının listesi (Image sınıfı örnekleri) (0-2)<br>
  Örnek: `pyxel.images[0].load(0, 0, "title.png")`

- `tilemaps`<br>
  Döşeme haritalarının listesi (Tilemap sınıfı örnekleri) (0-7)

- `clip(x, y, w, h)`<br>
  Ekranın çizim alanını (`x`, `y`) koordinatlarından genişlik `w` ve yükseklik `h` olarak ayarlar. Çizim alanını tam ekran olarak sıfırlamak için `clip()` çağrılır.

- `camera(x, y)`<br>
  Ekranın sol üst köşesinin koordinatlarını (`x`, `y`) olarak değiştirir. Sol üst köşe koordinatlarını (`0`, `0`) olarak sıfırlamak için `camera()` çağrılır.

- `pal(col1, col2)`<br>
  Çizim sırasında `col1` rengini `col2` ile değiştirir. Başlangıç paletine sıfırlamak için `pal()` çağrılır.

- `dither(alpha)`<br>
  Çizim sırasında dithering (sahte şeffaflık) uygular. `alpha` değerini `0.0` ile `1.0` arasında ayarlayın, `0.0` tamamen şeffaf, `1.0` ise opaktır.

- `cls(col)`<br>
  Ekranı `col` rengiyle temizler.

- `pget(x, y)`<br>
  (`x`, `y`) noktasındaki pikselin rengini alır.

- `pset(x, y, col)`<br>
  (`x`, `y`) noktasına `col` renginde bir piksel çizer.

- `line(x1, y1, x2, y2, col)`<br>
  (`x1`, `y1`) ile (`x2`, `y2`) arasında `col` renginde bir çizgi çizer.

- `rect(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından `w` genişlik ve `h` yükseklik ile `col` renginde bir dikdörtgen çizer.

- `rectb(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından `w` genişlik ve `h` yükseklik ile `col` renginde bir dikdörtgenin dış çizgilerini çizer.

- `circ(x, y, r, col)`<br>
  (`x`, `y`) noktasına yarıçapı `r` olan `col` renginde bir daire çizer.

- `circb(x, y, r, col)`<br>
  (`x`, `y`) noktasına yarıçapı `r` olan `col` renginde bir dairenin dış çizgilerini çizer.

- `elli(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından `w` genişlik ve `h` yükseklik ile `col` renginde bir elips çizer.

- `ellib(x, y, w, h, col)`<br>
  (`x`, `y`) noktasından `w` genişlik ve `h` yükseklik ile `col` renginde bir elipsin dış çizgilerini çizer.

- `tri(x1, y1, x2, y2, x3, y3, col)`<br>
  Tepeleri (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) olan `col` renginde bir üçgen çizer.

- `trib(x1, y1, x2, y2, x3, y3, col)`<br>
  Tepeleri (`x1`, `y1`), (`x2`, `y2`), (`x3`, `y3`) olan `col` renginde bir üçgenin dış çizgilerini çizer.

- `fill(x, y, col)`<br>
  (`x`, `y`) ile aynı renkle bağlanan alanı `col` rengiyle doldurur.

- `blt(x, y, img, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Görüntü bankası `img`(0-2) içinde (`u`, `v`) noktasından (`w`, `h`) boyutundaki bölgeyi (`x`, `y`) noktasına kopyalar. `w` ve/veya `h` için negatif bir değer atanırsa, bölge yatay ve/veya dikey olarak çevrilir. Eğer `colkey` belirtilirse, şeffaf renk olarak kabul edilir. Eğer `rotate`(derece olarak), `scale`(1.0 = %100) veya her ikisi belirtilirse, uygun dönüşümler uygulanır.

<img src="images/blt_figure.png">

- `bltm(x, y, tm, u, v, w, h, [colkey], [rotate], [scale])`<br>
  Karo Haritası `tm` (0-7) içindeki (`u`, `v`) konumundan başlayarak (`w`, `h`) boyutundaki bölgeyi (`x`, `y`) konumuna kopyalar. `w` ve/veya `h` için negatif bir değer atanırsa, bölge yatay ve/veya dikey olarak çevrilir. Eğer `colkey` belirtilirse, şeffaf renk olarak kabul edilir. Eğer `rotate` (derece cinsinden), `scale` (1.0 = %100) veya her ikisi belirtilirse, uygun dönüşümler uygulanır. Bir döşemenin boyutu 8x8 pikseldir ve döşeme haritasında `(image_tx, image_ty)` şeklinde bir ikili olarak saklanır.

<img src="images/bltm_figure.png">

- `text(x, y, s, col)`<br>
  `s` metnini, `col` renginde (`x`, `y`) noktasına çizer.

### Ses

- `sounds`<br>
  Seslerin listesi (Sound sınıfı örnekleri) (0-63)<br>
  Örnek: `pyxel.sounds[0].speed = 60`

- `musics`<br>
  Müziklerin listesi (Music sınıfı örnekleri) (0-7)

- `play(ch, snd, [tick], [loop], [resume])`<br>
  Kanal `ch`(0-3) üzerinde `snd`(0-63) sesini çalar. Eğer `snd` bir listeyse, sesler sırayla çalınır. Çalma başlangıç konumu `tick` (1 tick = 1/120 saniye) ile belirtilebilir. Eğer `loop` `True` olarak ayarlanmışsa, döngüsel çalma gerçekleştirilir. Çalma bittikten sonra önceki sese devam etmek için `resume` `True` olarak ayarlanır.

- `playm(msc, [tick], [loop])`<br>
  Müziği `msc`(0-7) çalar. Çalma başlangıç konumu `tick` (1 tick = 1/120 saniye) ile belirtilebilir. Eğer `loop` `True` olarak ayarlanmışsa, döngüsel çalma gerçekleştirilir.

- `stop([ch])`<br>
  Belirtilen `ch`(0-3) kanalındaki çalmayı durdurur. Tüm kanalların çalmasını durdurmak için `stop()` çağrılır.

- `play_pos(ch)`<br>
  Kanal `ch`(0-3) çalma pozisyonunu `(sound_no, note_no)` ikilisi olarak döndürür. Çalma durduğunda `None` döner.

### Matematik

- `ceil(x)`<br>
  `x`'ten büyük veya ona eşit en küçük tamsayıyı döndürür.

- `floor(x)`<br>
  `x`'ten küçük veya ona eşit en büyük tamsayıyı döndürür.

- `sgn(x)`<br>
  `x` pozitif olduğunda `1`, `0` olduğunda `0`, negatif olduğunda `-1` döndürür.

- `sqrt(x)`<br>
  `x`'in karekökünü döndürür.

- `sin(deg)`<br>
  `deg` derece için sinüs değerini döndürür.

- `cos(deg)`<br>
  `deg` derece için kosinüs değerini döndürür.

- `atan2(y, x)`<br>
  `y`/`x`'in ters tanjantını derece olarak döndürür.

- `rseed(seed)`<br>
  Rastgele sayı üreticisinin tohumunu ayarlar.

- `rndi(a, b)`<br>
  `a` ile `b` arasında rastgele bir tamsayı döndürür.

- `rndf(a, b)`<br>
  `a` ile `b` arasında rastgele bir ondalıklı sayı döndürür.

- `nseed(seed)`<br>
  Perlin gürültüsü için tohum ayarlar.

- `noise(x, [y], [z])`<br>
  Belirtilen koordinatlar için Perlin gürültü değerini döndürür.

### Image Sınıfı

- `width`, `height`<br>
  Görüntünün genişliği ve yüksekliği

- `set(x, y, data)`<br>
  (`x`, `y`) konumuna bir dizi dize kullanarak görüntü ayarlar.<br>
  Örnek: `pyxel.images[0].set(10, 10, ["0123", "4567", "89ab", "cdef"])`

- `load(x, y, filename)`<br>
  (`x`, `y`) konumuna bir görüntü dosyasını (PNG/GIF/JPEG) yükler.

- `pget(x, y)`<br>
  (`x`, `y`) konumundaki pikselin rengini alır.

- `pset(x, y, col)`<br>
  (`x`, `y`) konumuna `col` renginde bir piksel çizer.

### Tilemap Sınıfı

- `width`, `height`<br>
  Döşeme haritasının genişliği ve yüksekliği

- `imgsrc`<br>
  Döşeme haritasının referans aldığı görüntü bankası (0-2)

- `set(x, y, data)`<br>
  (`x`, `y`) konumuna bir dizi dize kullanarak döşeme haritası ayarlar.<br>
  Örnek: `pyxel.tilemap(0).set(0, 0, ["0000 0100 a0b0", "0001 0101 a1b1"])`

- `load(x, y, filename, layer)`<br>
  TMX dosyasından `layer`(0-) katmanını (`x`, `y`) konumuna yükler.

- `pget(x, y)`<br>
  (`x`, `y`) konumundaki döşemeyi alır. Döşeme, `(image_tx, image_ty)` ikilisi olarak temsil edilir.

- `pset(x, y, tile)`<br>
  (`x`, `y`) konumuna bir `tile` çizer. Döşeme, `(image_tx, image_ty)` ikilisi olarak temsil edilir.

### Sound Sınıfı

- `notes`<br>
  Notaların listesi (0-127). Sayı büyüdükçe perde yükselir. `33` notası 'A2' (440Hz) ile eşleşir. Sus notaları `-1` ile gösterilir.

- `tones`<br>
  Ses tonlarının listesi (0:Triangle / 1:Square / 2:Pulse / 3:Noise)

- `volumes`<br>
  Ses seviyelerinin listesi (0-7)

- `effects`<br>
  Efektlerin listesi (0:None / 1:Slide / 2:Vibrato / 3:FadeOut / 4:Half-FadeOut / 5:Quarter-FadeOut)

- `speed`<br>
  Çalma hızı. `1` en hızlısıdır ve sayı büyüdükçe çalma hızı yavaşlar. `120`'de, bir notanın süresi 1 saniyeye eşittir.

- `set(notes, tones, volumes, effects, speed)`<br>
  Bir dize kullanarak notaları, tonları, ses seviyelerini ve efektleri ayarlar. Eğer tonların, ses seviyelerinin veya efektlerin uzunluğu notalardan kısaysa, baştan itibaren tekrarlanır.

- `set_notes(notes)`<br>
  `CDEFGAB`+`#-`+`01234` veya `R` ile notaları ayarlar. Büyük/küçük harf duyarsızdır ve boşluklar göz ardı edilir.<br>
  Örnek: `pyxel.sounds[0].set_notes("g2b-2d3r rf3f3f3")`

- `set_tones(tones)`<br>
  `TSPN` dizesiyle tonları ayarlar. Büyük/küçük harf duyarsızdır ve boşluklar göz ardı edilir.<br>
  Örnek: `pyxel.sounds[0].set_tones("ttss pppn")`

- `set_volumes(volumes)`<br>
  `01234567` dizesiyle ses seviyelerini ayarlar. Büyük/küçük harf duyarsızdır ve boşluklar göz ardı edilir.<br>
  Örnek: `pyxel.sounds[0].set_volumes("7777 7531")`

- `set_effects(effects)`<br>
  `NSVFHQ` dizesiyle efektleri ayarlar. Büyük/küçük harf duyarsızdır ve boşluklar göz ardı edilir.<br>
  Örnek: `pyxel.sounds[0].set_effects("nfnf nvvs")`

- `mml(code)`<br>
  [Music Macro Language (MML)](https://en.wikipedia.org/wiki/Music_Macro_Language) kullanarak ilgili parametreleri ayarlar. Kullanılabilir komutlar `T`(1-900), `@`(0-3), `O`(0-4), `>`, `<`, `Q`(1-8), `V`(0-7), `X`(0-7), `L`(1/2/4/8/16/32) ve `CDEFGABR`+`#+-`+`.~&`. Komutlar hakkında daha fazla bilgi için [bu sayfayı](faq-en.md) inceleyin.<br>
  Örnek: `pyxel.sounds[0].mml("t120 @1 o3 q6 l8 x0:12345 c4&c<g16r16>c.<g16 v4 >c.&d16 x0 e2~c2~")`

- `save(filename, count, [ffmpeg])`<br>
  Sesi `count` kez tekrarlayan bir WAV dosyası oluşturur. FFmpeg yüklüyse ve `ffmpeg` `True` olarak ayarlandıysa, bir MP4 dosyası da oluşturulur.

- `total_sec()`<br>
  Sesi saniye cinsinden çalma süresini döndürür. MML'de sonsuz döngü kullanılmışsa `None` döndürür.

### Music Sınıfı

- `seqs`<br>
  Birden fazla kanal boyunca seslerin (0-63) iki boyutlu listesi

- `set(seq0, seq1, seq2, ...)`<br>
  Her kanal için ses listelerini (0-63) ayarlar. Boş bir liste belirtilirse, o kanal çalma için kullanılmaz.<br>
  Örnek: `pyxel.musics[0].set([0, 1], [], [3])`

- `save(filename, count, [ffmpeg])`<br>
  Müziği `count` kez tekrarlayan bir WAV dosyası oluşturur. FFmpeg yüklüyse ve `ffmpeg` `True` olarak ayarlandıysa, bir MP4 dosyası da oluşturulur.

### Gelişmiş API

Pyxel, kullanıcıları yanıltma veya kullanmak için özel bilgi gerektirme potansiyeli nedeniyle, bu referansta yer almayan "Gelişmiş API" içerir.

Eğer yeteneklerinize güveniyorsanız, [bu](../python/pyxel/__init__.pyi) rehber olarak kullanarak harika eserler yaratmayı deneyin!

## Katkıda Bulunma

### Sorun Bildirme

Hata raporları ve özellik veya iyileştirme talepleri göndermek için [Sorun Takip Sistemi](https://github.com/kitao/pyxel/issues) kullanın. Yeni bir sorun göndermeden önce, benzer açık sorunlar olmadığından emin olun.

### Fonksiyonel Test

Kodu manuel olarak test eden ve [Sorun Takip Sistemi](https://github.com/kitao/pyxel/issues) üzerinden hata veya iyileştirme önerileri bildiren herkes çok memnun karşılanır!

### Pull İsteği Gönderme

Yamanlar ve düzeltmeler, pull isteği (PR) şeklinde kabul edilmektedir. Pull isteğinin ele aldığı sorunun Sorun Takip Sistemi'nde açık olduğundan emin olun.

Pull isteği göndermek, katkınızı [MIT Lisansı](../LICENSE) altında lisanslamayı kabul ettiğiniz anlamına gelir.

## Diğer Bilgiler

- [Sıkça Sorulan Sorular](faq-en.md)
- [Kullanıcı Örnekleri](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Geliştiricinin X Hesabı](https://x.com/kitao)
- [Discord Sunucusu (İngilizce)](https://discord.gg/Z87eYHN)
- [Discord Sunucusu (Japonca)](https://discord.gg/qHA5BCS)

## Lisans

Pyxel, [MIT Lisansı](../LICENSE) ile lisanslanmıştır. Tüm kopyalarındaki yazılım veya onun önemli bölümleri, MIT Lisansı koşullarını ve telif hakkı bildirimini içermesi koşuluyla özel yazılımda kullanılabilir.

## Sponsor Arayışı

Pyxel, GitHub Sponsors üzerinde sponsorlar arıyor. Pyxel'in sürdürülebilir bakımı ve özellik geliştirmesi için sponsor olmayı düşünün. Bir avantaj olarak, sponsorlar Pyxel geliştiricisiyle doğrudan danışma imkânına sahip olabilirler. Daha fazla bilgi için [bu sayfayı](https://github.com/sponsors/kitao) ziyaret edin.
