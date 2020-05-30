from setuptools import setup

setup(
    name='reproduction',
    version='0.1',
    #py_modules=['cli'],
    #packages=['jellyfish'],
    # Does a little bit of magic which creates a `jellyfish` command
    # we can run from the command line.
    # See https://click.palletsprojects.com/en/7.x/setuptools/ for more info
    #entry_points='''
    #    [console_scripts]
    #    jellyfish=jellyfish.cli:main
    #''',
    # We're not releasing this package, so we don't need to list its
    # dependencies. If you were going to release a package (could be a good idea
    # if you have an experiment you'd like to share!), you would list the
    # dependencies here.
    install_requires=[],
)
