# dyn-a-wall

Create dynamic wallpapers (aka background slide shows) for **Gnome Shell**

🦀🦀🦀 [![Rust](https://img.shields.io/badge/Rust-1.93-orange?style=for-the-badge)](https://rust-lang.org/) 🦀🦀🦀

> [!IMPORTANT]
> Work in progress!  This app is not ready for use yet.


>[!NOTE]
> **AI Usage Disclosure:**
> This project is entirely hand-crafted by a human 💖.  No AI tools of any kind were used in any
> stage of the development cycle.  Ideas, concepts, research, design, programming, all of it is made
> by an actual human being (with some help of other humans).


# What is this?

**dyn-a-wall** (dˈaɪnʌwˌɔːl) _(dynamic wallpaper)_ let's you create and optionally install "dynamic
wallpapers" (or background slide shows) for **Gnome Shell**.  These are backgrounds/wallpapers that
change automatically after a specified period of time with optional transitions between images.

> _**I don't wanna readd all of this! Hurry up and just tell me how to make my dynamic wallpaper!**_
>
> _In a nutshell, assuming you've already downloaded and compiled/installed **dynawall**, just run
> the following command to create and install the wallpaper:_
>
> `$ dynawall -d /path/to/my/images -i`
>
> _(Don't  type the `$`, that's your shell prompt.)  If nothing goes wrong, that command will create
> the wallpaper with the default settings and install it under `/home/${USER}/.local/share/`.  For
> more options read [**How do I use it?**](#how-do-i-use-it)._


# Why is this useful?

> **TL;DR**:
> _It's a quick way to turn a bunch of images into a background slide show and make it instantly
> available in gnome settings._

The way Gnome Shell handles that is by reading some XML files describing the wallpaper and the slide
show.  If you have a bunch of images that you may want to use as a background slide show, putting it
all up together is a little bit tedious and error prone.  On top of that, you may end up needlessly 
duplicating the images and polluting the shared directory.

This program does it all in one go (no, not _duplicating the images and polluting the shared
directory_ 😅 ... I mean it _creates the files and folders and sets it all up_ 🤓), including
installing the wallpaper and effectively making it instantly available in Gnome Settings.  It does
that in such a way that you don't need to duplicate your images (unless you want to (and you may
have a good reason for doing so (keep reading I'll get back to this))) and making it easy to
uninstall without leaving any files behind (even if you opt for copying the images).  You can even
make a copy of the wallpaper (or directly generate it on its own directory) and install it on a
different machine.


# Why did I do this?

I tend to do most of my work (like 99% of it) on the terminal and I prefer to have it configured
with a translucent background (bear with me).  It's just more enjoyable for me.  As you can imaging,
I end up seeing the wallpaper image _all the time_.  No wonder I like to change that image from time
to time.

On Gnome forty  something (sorry I don't remember the exact version) _gnome-backgrounds_ included
some adaptive backgrounds and a few _time-of-day_ backgrounds as well.  Reading the xml files of
those I came up with the idea of creating my "automatic wallpaper changer" (or "slide show") for the
Shell.  Since I _failed to find (official) documentation about that feature_, I just use one of
those _time-of-day_ xml files as a reference and created my own _timed_ wallpaper.

On recent versions of **the Shell** (and gnome-backgrounds) they're not present any more, but the
feature still works (I hope they don't remove it) so I kept making my own timed wallpapers.  Back
then when I did this for the first time I didn't know much about programming but now that I've
acquired some experience it occurred to me that this was not only very tedious to edit by hand but
also very easy to automate.  It's also a good opportunity to keep learning and to share my findings
with that other one person that is a little bit like me and also wants to do this _(says slightly
joking)_.


# How do I use it?

> [Work in progress]


# How does it work?

> [Work in progress]


# Aim of this tool

## Initial version

Simple command with arguments to create and/or install the wallpaper.

Input (command line arguments or flags):

- Where to find the images, or as we call it, a `path`.
- Whether or not to use transitions between images, in which case:
    - Duration of that transition.
    - Type of transition? _(if I manage to find some docs about it)_
- How long to wait before moving to the next image.
- Whether to install the wallpaper or generate it on a particular directory for later use.
- Whether to link to images or copy them in the wallpaper directory.


## Next version

CLI tool with interactive selection of different options.

> [Work in progress]

---

If you've read this far, thanks!  I really appreciate it and I hope you'll find this useful.

Happy _dynawall-ing_ 🥰!

> _**Nota bene:** I know, I know.  That's not even a word!_
