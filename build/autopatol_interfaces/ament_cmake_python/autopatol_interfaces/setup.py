from setuptools import find_packages
from setuptools import setup

setup(
    name='autopatol_interfaces',
    version='0.0.0',
    packages=find_packages(
        include=('autopatol_interfaces', 'autopatol_interfaces.*')),
)
