from setuptools import find_packages, setup
from glob import glob

package_name = 'autopartol_robot'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
         ['resource/' + package_name]),

        ('share/' + package_name, ['package.xml']),

        ('share/' + package_name + '/config',
         glob('config/*.yaml')),

        ('share/' + package_name + '/launch',
         glob('launch/*.launch.py')),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='fishros',
    maintainer_email='87068644+fishros@users.noreply.github.com',
    description='ROS2 patrol robot',
    license='TODO',
    tests_require=['pytest'],
    entry_points={
        'console_scripts': [
            'patrol_node = autopartol_robot.patrol_node:main',
            'speaker = autopartol_robot.speaker:main',
        ],
    },
)