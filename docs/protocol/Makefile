INKSCAPE = /Applications/Inkscape.app/Contents/MacOS/inkscape

%.pdf: %.svg
	$(INKSCAPE) $^ --export-area-drawing --export-filename $@

intermediate.pdf: intermediate.tex #gcow.pdf
	pdflatex --shell-escape intermediate.tex

clean:
	rm -f *.aux *.pdf *.dvi *.log *~
