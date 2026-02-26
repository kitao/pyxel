# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/), Python için bir retro oyun motorudur.

Özellikler, yalnızca 16 renk desteği ve 4 ses kanalıyla retro oyun konsollarından ilham alınarak tasarlanmıştır, böylece piksel sanat tarzı oyunlar yapmayı kolayca keyifle yaşayabilirsiniz.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

Pyxel'in geliştirilmesi, kullanıcı geri bildirimleriyle yönlendirilmektedir. Lütfen GitHub'da Pyxel'e bir yıldız verin!

<p>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/10-platformer.html">
<img src="images/10_platformer.gif" width="290">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/30sec-of-daylight.html">
<img src="images/30sec_of_daylight.gif" width="350">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/examples/02-jump-game.html">
<img src="images/02_jump_game.gif" width="330">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/apps/megaball.html">
<img src="images/megaball.gif" width="310">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_tilemap_editor.gif" width="320">
</a>
<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
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
- 3 adet 256x256 görüntü bankası
- 8 adet 256x256 döşeme haritası
- 64 tanımlanabilir ses ile 4 kanal
- Herhangi bir sesi birleştirebilen 8 müzik parçası
- Klavye, fare ve gamepad girişi
- Görüntü ve ses düzenleme araçları
- Kullanıcı tarafından genişletilebilir renkler, ses kanalları ve bankalar

### Renk Paleti

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Nasıl Kurulur

### Windows

[Python 3](https://www.python.org/) (3.8 veya daha yüksek sürüm) kurduktan sonra, aşağıdaki komutu çalıştırın:

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

[Python 3](https://www.python.org/) (3.8 veya daha yüksek sürüm) kurduktan sonra, aşağıdaki komutu çalıştırın:

```sh
pip install -U pyxel
```

Yukarıdaki komut çalışmazsa, Pyxel'i kaynak kodundan inşa etmeyi düşünün ve [Makefile](../Makefile) içindeki talimatları izleyin.

### Web

Pyxel'in web sürümü, uyumlu bir tarayıcıyla PC, akıllı telefon ve tablette, Python veya Pyxel yüklemeden kullanılabilir.

Kullanmanın en kolay yolu, çevrimiçi IDE [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) aracılığıyladır.

Kendi sitenizde Pyxel uygulamalarını gömme gibi diğer kullanım modelleri için, lütfen [bu sayfaya](pyxel-web-en.md) bakın.

## Temel Kullanım

### Pyxel Komutu

Pyxel'i yüklemek `pyxel` komutunu ekler. Çeşitli işlemler gerçekleştirmek için `pyxel`'den sonra bir komut adı belirtin.

Mevcut komutların listesini görmek için argümansız çalıştırın:

```sh
pyxel
```

```
Pyxel 2.7.1, a retro game engine for Python
usage:
    pyxel run PYTHON_SCRIPT_FILE(.py)
    pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE(.py)
    pyxel play PYXEL_APP_FILE(.pyxapp)
    pyxel edit [PYXEL_RESOURCE_FILE(.pyxres)]
    pyxel package APP_DIR STARTUP_SCRIPT_FILE(.py)
    pyxel app2exe PYXEL_APP_FILE(.pyxapp)
    pyxel app2html PYXEL_APP_FILE(.pyxapp)
    pyxel copy_examples
```

### Örnekleri Çalıştırma

Aşağıdaki komutla örnekleri geçerli dizine kopyalayabilirsiniz:

```sh
pyxel copy_examples
```

Yerel ortamda örnekler aşağıdaki komutlarla çalıştırılabilir:

```sh
# examples dizinindeki örneği çalıştırın
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# examples/apps dizinindeki uygulamayı çalıştırın
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Örnek listesi [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/) üzerinden tarayıcıda da görüntülenebilir ve çalıştırılabilir.

## Uygulama Oluşturma

### Program Oluşturma

Python betiğinizde Pyxel'i içe aktarın, pencere boyutunu `init` ile belirtin ve uygulamayı `run` ile başlatın.

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

Dizin izlemeyi `Ctrl(Command)+C` tuşlarına basarak durdurun.

### Özel Tuş Operasyonları

Bir Pyxel uygulaması çalışırken, aşağıdaki özel tuş işlemleri gerçekleştirilebilir:

- `Esc`<br>
  Uygulamadan çık
- `Alt(Option)+R` veya gamepad'de `A+B+X+Y+BACK`<br>
  Uygulamayı sıfırla
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

## Kaynak Oluşturma

### Pyxel Editor

Pyxel Editor, Pyxel uygulamalarında kullanılan resim ve sesleri oluşturur.

Pyxel Editor'ü aşağıdaki komut ile başlatabilirsiniz:

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Belirtilen Pyxel kaynak dosyası (.pyxres) mevcutsa, yüklenecektir. Eğer mevcut değilse, belirtilen isimle yeni bir dosya oluşturulacaktır. Kaynak dosyası belirtilmezse, `my_resource.pyxres` adıyla yeni bir dosya oluşturulacaktır.

Pyxel Editor başlatıldıktan sonra, başka bir kaynak dosyasına geçmek için dosyayı editöre sürükleyip bırakabilirsiniz.

Oluşturulan kaynak dosyası, `load` fonksiyonu ile yüklenebilir.

Pyxel Editor aşağıdaki düzenleme modlarına sahiptir.

**Resim Editörü**

Her **görüntü bankasındaki** görüntüyü düzenlemek için kullanılan mod.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/image-editor.html">
<img src="images/image_editor.gif">
</a>

Resim editörüne bir resim dosyası (PNG/GIF/JPEG) sürükleyip bırakarak, görüntüyü şu anda seçili olan görüntü bankasına yükleyebilirsiniz.

**Döşeme Haritası Editörü**

Görüntü bankalarındaki görüntüleri bir **döşeme haritası** içinde düzenlemek için kullanılan mod.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/tilemap-editor.html">
<img src="images/tilemap_editor.gif">
</a>

Bir TMX dosyasını (Tiled Map File) döşeme haritası editörüne sürükleyip bırakarak, şu anda seçili olan döşeme haritasına katman 0'ı yükleyebilirsiniz.

**Ses Editörü**

Melodi ve **ses** efektlerinde kullanılan sesleri düzenlemek için mod.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/sound-editor.html">
<img src="images/sound_editor.gif">
</a>

**Müzik Editörü**

Seslerin çalma sırasına göre dizildiği **müzik parçalarını** düzenlemek için kullanılan mod.

<a href="https://kitao.github.io/pyxel/wasm/showcase/tools/music-editor.html">
<img src="images/music_editor.gif">
</a>

### Diğer Kaynak Oluşturma Yöntemleri

Pyxel resimleri ve döşeme haritaları aşağıdaki yöntemlerle de oluşturulabilir:

- `Image.set` veya `Tilemap.set` fonksiyonlarıyla dize listelerinden resim veya döşeme haritası oluşturun
- `Image.load` fonksiyonuyla Pyxel paletine uygun bir resim dosyasını (PNG/GIF/JPEG) yükleyin

Pyxel sesleri ve müzikleri de aşağıdaki yöntemle oluşturulabilir:

- `Sound.set` veya `Music.set` fonksiyonlarıyla dizelerden oluşturun

Bu fonksiyonların kullanımı için API referansına bakın.

## Uygulama Dağıtımı

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

## API Referansı

Pyxel API'lerinin tam listesi [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/) adresinde mevcuttur.

Pyxel ayrıca uzmanlık bilgisi gerektiren bir "Gelişmiş API" içerir. Referans sayfasında "Advanced" onay kutusunu işaretleyerek görüntüleyebilirsiniz.

Yeteneklerinize güveniyorsanız, Gelişmiş API'yi kullanarak gerçekten şaşırtıcı eserler yaratmayı deneyin!

## Katkıda Bulunma

### Sorun Bildirme

Hata raporları ve özellik veya iyileştirme talepleri göndermek için [Sorun Takip Sistemi](https://github.com/kitao/pyxel/issues) kullanın. Yeni bir sorun göndermeden önce, benzer açık sorunlar olmadığından emin olun.

### Fonksiyonel Test

Kodu manuel olarak test eden ve [Sorun Takip Sistemi](https://github.com/kitao/pyxel/issues) üzerinden hata veya iyileştirme önerileri bildiren herkes çok memnun karşılanır!

### Pull İsteği Gönderme

Yamanlar ve düzeltmeler, pull isteği (PR) şeklinde kabul edilmektedir. Pull isteğinin ele aldığı sorunun Sorun Takip Sistemi'nde açık olduğundan emin olun.

Pull isteği göndermek, katkınızı [MIT Lisansı](../LICENSE) altında lisanslamayı kabul ettiğiniz anlamına gelir.

## Web Araçlar ve Örnekler

- [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Pyxel Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) [[User Manual](https://qiita.com/kitao/items/b5b3fb28ebf9781eda2e)]
- [Pyxel MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/) [[User Manual](https://qiita.com/kitao/items/a86de4f7d6a0ed656a89)]

## Diğer Bilgiler

- [Sıkça Sorulan Sorular](faq-en.md)
- [Kullanıcı Örnekleri](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Geliştiricinin X Hesabı](https://x.com/kitao)
- [Discord Sunucusu (İngilizce)](https://discord.gg/Z87eYHN)
- [Discord Sunucusu (Japonca)](https://discord.gg/qHA5BCS)

## Lisans

Pyxel, [MIT Lisansı](../LICENSE) ile lisanslanmıştır. Tüm kopyalardındaki yazılım veya onun önemli bölümleri, MIT Lisansı koşullarını ve telif hakkı bildirimini içermesi koşuluyla özel yazılımda kullanılabilir.

## Sponsor Arayışı

Pyxel, GitHub Sponsors üzerinde sponsorlar arıyor. Pyxel'in sürdürülebilir bakımı ve özellik geliştirmesi için sponsor olmayı düşünün. Bir avantaj olarak, sponsorlar Pyxel geliştiricisiyle doğrudan danışma imkânına sahip olabilirler. Daha fazla bilgi için [bu sayfayı](https://github.com/sponsors/kitao) ziyaret edin.
