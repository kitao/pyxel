# <img src="images/pyxel_logo_152x64.png">

[![Downloads](https://static.pepy.tech/personalized-badge/pyxel?period=total&units=international_system&left_color=grey&right_color=blue&left_text=PyPI%20downloads)](https://pypi.org/project/pyxel/)
[![GitHub Repo stars](https://img.shields.io/github/stars/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub forks](https://img.shields.io/github/forks/kitao/pyxel?style=social)](https://github.com/kitao/pyxel)
[![GitHub Sponsors](https://img.shields.io/github/sponsors/kitao?label=Sponsor%20me&logo=github%20sponsors&style=social)](https://github.com/sponsors/kitao)

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/H2H27VDKD)

[ [English](../README.md) | [中文](README.cn.md) | [Deutsch](README.de.md) | [Español](README.es.md) | [Français](README.fr.md) | [Italiano](README.it.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Português](README.pt.md) | [Русский](README.ru.md) | [Türkçe](README.tr.md) | [Українська](README.uk.md) ]

**Pyxel** (/ˈpɪksəl/) é um motor de jogos retro para Python.

As especificações são inspiradas em consoles de jogos retro, como o suporte para apenas 16 cores e 4 canais de som, permitindo que você desfrute facilmente da criação de jogos em estilo pixel art.

[<img src="images/pyxel_thanks.png" width="460">](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples) [<img src="images/pyxel_book.png" width="180">](https://gihyo.jp/book/2025/978-4-297-14657-3)

O desenvolvimento do Pyxel é impulsionado pelo feedback dos usuários. Por favor, dê uma estrela ao Pyxel no GitHub!

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

As especificações e APIs do Pyxel são inspiradas no [PICO-8](https://www.lexaloffle.com/pico-8.php) e no [TIC-80](https://tic80.com/).

O Pyxel é de código aberto sob a [Licença MIT](../LICENSE) e é gratuito para usar. Vamos começar a criar jogos retrô com o Pyxel!

## Especificações

- Funciona no Windows, Mac, Linux e Web
- Programação em Python
- Tamanho de tela personalizável
- Paleta de 16 cores
- 3 bancos de imagem 256x256
- 8 mapas de ladrilhos 256x256
- 4 canais com 64 sons definíveis
- 8 faixas de música capazes de combinar quaisquer sons
- Entradas de teclado, mouse e gamepad
- Ferramentas de edição de imagens e sons
- Cores, canais de som e bancos extensíveis pelo usuário

### Paleta de cores

<img src="images/05_color_palette.png">

<img src="images/pyxel_palette.png">

## Como instalar

### Windows / Mac / Linux

Após instalar o [Python 3](https://www.python.org/) (versão 3.8 ou superior), execute o seguinte comando:

```sh
pip install -U pyxel
```

**Nota:** No Windows, certifique-se de marcar `Add python.exe to PATH` ao instalar o Python para habilitar o comando `pyxel`.

### Web

A versão web do Pyxel pode ser usada sem instalação em PCs, smartphones, tablets e outros dispositivos com um navegador compatível.

O ambiente de desenvolvimento web [Pyxel Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/) está pronto para uso, basta abri-lo no navegador.

Para outros usos, como incorporar aplicativos Pyxel em seu próprio site, consulte [esta página](pyxel-web-en.md).

### VS Code

Adicionando a extensão Pyxel ao [Visual Studio Code](https://code.visualstudio.com/) (VS Code), você pode desenvolver e executar aplicativos Pyxel sem instalar Python ou Pyxel.

Para adicionar a extensão Pyxel, pesquise por "[Pyxel](https://marketplace.visualstudio.com/items?itemName=kitao.pyxel-vscode)" na visualização de Extensões do VS Code e clique no botão de instalação.

## Uso básico

### Comando Pyxel

A instalação do Pyxel adiciona o comando `pyxel`. Especifique um nome de comando após `pyxel` para realizar diversas operações.

Execute-o sem argumentos para ver a lista de comandos disponíveis:

```sh
pyxel
```

```
Pyxel, a retro game engine for Python
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

### Executar exemplos

O seguinte comando copia os exemplos do Pyxel para o diretório atual:

```sh
pyxel copy_examples
```

No ambiente local, os exemplos podem ser executados com os seguintes comandos:

```sh
# Executar exemplo no diretório examples
cd pyxel_examples
pyxel run 01_hello_pyxel.py

# Executar app no diretório examples/apps
cd apps
pyxel play 30sec_of_daylight.pyxapp
```

Os exemplos também podem ser visualizados e executados no navegador a partir do [Pyxel Showcase](https://kitao.github.io/pyxel/wasm/showcase/).

## Criação de aplicações

### Criar um programa

No seu script Python, importe o Pyxel, especifique o tamanho da janela com `init` e inicie o aplicativo com `run`.

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

Os argumentos da função `run` são a função `update`, que processa as atualizações de quadro, e a função `draw`, que lida com a exibição na tela.

Em um aplicativo real, é recomendável encapsular o código Pyxel em uma classe, como mostrado abaixo:

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

Para criar gráficos simples sem animação, você pode usar a função `show` para simplificar seu código.

```python
import pyxel

pyxel.init(120, 120)
pyxel.cls(1)
pyxel.circb(60, 60, 40, 7)
pyxel.show()
```

### Executar um programa

Um script criado pode ser executado usando o comando `python`:

```sh
python PYTHON_SCRIPT_FILE
```

Ele também pode ser executado com o comando `pyxel run`:

```sh
pyxel run PYTHON_SCRIPT_FILE
```

Além disso, o comando `pyxel watch` monitora alterações em um diretório especificado e executa automaticamente o programa quando mudanças são detectadas:

```sh
pyxel watch WATCH_DIR PYTHON_SCRIPT_FILE
```

Interrompa a vigilância do diretório pressionando `Ctrl(Command)+C`.

### Controles de teclas especiais

Durante a execução de uma aplicação Pyxel, as seguintes operações de teclas especiais podem ser realizadas:

- `Esc`<br>
  Sair do aplicativo
- `Alt(Option)+R` ou `A+B+X+Y+BACK` no gamepad<br>
  Reiniciar o aplicativo
- `Alt(Option)+1`<br>
  Salvar a captura de tela na área de trabalho
- `Alt(Option)+2`<br>
  Reiniciar o tempo de início da gravação do vídeo de captura de tela
- `Alt(Option)+3`<br>
  Salvar um vídeo de captura de tela na área de trabalho (até 10 segundos)
- `Alt(Option)+8` ou `A+B+X+Y+DL` no gamepad<br>
  Alternar o modo de escala da tela entre máximo e inteiro
- `Alt(Option)+9` ou `A+B+X+Y+DR` no gamepad<br>
  Alternar entre os modos de tela (Crisp/Smooth/Retro)
- `Alt(Option)+0` ou `A+B+X+Y+DU` no gamepad<br>
  Alternar o monitor de desempenho (FPS/tempo de `update`/tempo de `draw`)
- `Alt(Option)+Enter` ou `A+B+X+Y+DD` no gamepad<br>
  Alternar para tela cheia
- `Shift+Alt(Option)+1/2/3`<br>
  Salvar o banco de imagens 0, 1 ou 2 na área de trabalho
- `Shift+Alt(Option)+0`<br>
  Salvar a paleta de cores atual na área de trabalho

## Criação de recursos

### Pyxel Editor

O Pyxel Editor cria imagens e sons usados em uma aplicação Pyxel.

```sh
pyxel edit PYXEL_RESOURCE_FILE
```

Para instruções detalhadas, consulte o [Manual do Pyxel Editor](https://kitao.github.io/pyxel/wasm/editor-manual/).

### Outros métodos de criação

Imagens e mapas de ladrilhos do Pyxel também podem ser criados usando os seguintes métodos:

- Crie imagens ou mapas de ladrilhos a partir de listas de strings com as funções `Image.set` ou `Tilemap.set`
- Carregue arquivos de imagem compatíveis com a paleta do Pyxel (PNG/GIF/JPEG) com a função `Image.load`

Os sons e as músicas do Pyxel também podem ser criados usando o seguinte método:

- Crie-os a partir de strings com as funções `Sound.set` ou `Music.set`

Consulte a referência da API para o uso dessas funções.

## Distribuição de aplicações

O Pyxel suporta um formato de distribuição multiplataforma chamado arquivo de aplicativo Pyxel.

Crie um arquivo de aplicativo Pyxel (.pyxapp) com o comando `pyxel package`:

```sh
pyxel package APP_DIR STARTUP_SCRIPT_FILE
```

Se você precisar incluir recursos ou módulos adicionais, coloque-os no diretório do aplicativo.

Os metadados podem ser exibidos em tempo de execução, especificando-os no seguinte formato dentro do script de inicialização. Os campos além de `title` e `author` são opcionais.

```python
# title: Pyxel Platformer
# author: Takashi Kitao
# desc: A Pyxel platformer example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0
```

O arquivo de aplicativo criado pode ser executado usando o comando `pyxel play`:

```sh
pyxel play PYXEL_APP_FILE
```

Um arquivo de aplicativo Pyxel também pode ser convertido em um executável ou um arquivo HTML usando os comandos `pyxel app2exe` ou `pyxel app2html`.

## Referência da API

A lista completa das APIs do Pyxel está disponível em [Pyxel API Reference](https://kitao.github.io/pyxel/wasm/api-reference/).

O Pyxel também inclui uma "API avançada" que requer conhecimento especializado. Você pode visualizá-la marcando a caixa "Advanced" na página de referência.

Se você confia em suas habilidades, tente usar a API avançada para criar obras verdadeiramente surpreendentes!

## Como Contribuir

### Enviando Problemas

Use o [Rastreador de Problemas](https://github.com/kitao/pyxel/issues) para enviar relatórios de bugs e solicitações de recursos ou melhorias. Antes de enviar um novo problema, verifique se não há problemas semelhantes abertos.

### Testes Funcionais

Qualquer pessoa que teste manualmente o código e relate bugs ou sugestões de melhorias no [Rastreador de Problemas](https://github.com/kitao/pyxel/issues) é muito bem-vinda!

### Enviando Pull Requests

Patches e correções são aceitos na forma de pull requests (PRs). Certifique-se de que o problema que o pull request aborda está aberto no Rastreador de Problemas.

Enviar um pull request implica que você concorda em licenciar sua contribuição sob a [Licença MIT](../LICENSE).

## Ferramentas e Exemplos

- [Showcase](https://kitao.github.io/pyxel/wasm/showcase/)
- [API Reference](https://kitao.github.io/pyxel/wasm/api-reference/)
- [Editor Manual](https://kitao.github.io/pyxel/wasm/editor-manual/)
- [Web Launcher](https://kitao.github.io/pyxel/wasm/launcher/)
- [Code Maker](https://kitao.github.io/pyxel/wasm/code-maker/)
- [MML Studio](https://kitao.github.io/pyxel/wasm/mml-studio/)
- [VS Code Extension](https://marketplace.visualstudio.com/items?itemName=kitao.pyxel-vscode)
- [MCP Server](https://github.com/kitao/pyxel-mcp)
- [Claude Code Skill](https://github.com/kitao/pyxel-skill)

## Outras Informações

- [FAQ](faq-en.md)
- [Exemplos de Usuários](https://github.com/kitao/pyxel/wiki/Pyxel-User-Examples)
- [Conta X do Desenvolvedor](https://x.com/kitao)
- [Servidor do Discord (Inglês)](https://discord.gg/Z87eYHN)
- [Servidor do Discord (Japonês)](https://discord.gg/qHA5BCS)

## Licença

O Pyxel é licenciado sob a [Licença MIT](../LICENSE). Pode ser reutilizado em software proprietário, desde que todas as cópias do software ou suas partes substanciais incluam uma cópia dos termos da Licença MIT e um aviso de copyright.

## Recrutamento de Patrocinadores

O Pyxel está buscando patrocinadores no GitHub Sponsors. Considere patrocinar o Pyxel para apoiar sua manutenção contínua e desenvolvimento de recursos. Como benefício, os patrocinadores podem consultar diretamente o desenvolvedor do Pyxel. Para mais detalhes, por favor, visite [esta página](https://github.com/sponsors/kitao).
