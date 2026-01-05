INKSCAPE = /Applications/Inkscape.app/Contents/MacOS/inkscape

%.pdf: %.svg
	$(INKSCAPE) $^ --export-area-drawing --export-filename $@

implementations.pdf: implementations.tex #gcow.pdf
	pdflatex --shell-escape implementations.tex

clean:
	rm -f *.aux *.pdf *.dvi *.log *~
