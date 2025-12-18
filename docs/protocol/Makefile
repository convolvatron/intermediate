INKSCAPE = /Applications/Inkscape.app/Contents/MacOS/inkscape

%.pdf: %.svg
	$(INKSCAPE) $^ --export-area-drawing --export-filename $@

protocol.pdf: protocol.tex entity.pdf
	pdflatex --shell-escape protocol.tex

clean:
	rm -f *.aux *.pdf *.dvi *.log *~
