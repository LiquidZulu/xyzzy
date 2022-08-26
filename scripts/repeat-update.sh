#!/usr/bin/env bash

while ! sudo pacman -Syyu ; do sleep 200 ; done ; xcalc
