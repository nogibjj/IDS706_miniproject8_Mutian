
pyinstall:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

pytest:
	python -m pytest -vv --cov=main test_*.py

pyformat:	
	black *.py 

pylint:
	pylint --disable=R,C --ignore-patterns=test_.*?py *.py
		
pyall: pyinstall pylint pyformat pytest 