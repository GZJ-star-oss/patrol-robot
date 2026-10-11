import os
import launch
import launch_ros
from ament_index_python.packages import get_package_share_directory


def generate_launch_description():

    # 1. 获取 ROS2 功能包安装目录
    autopartol_robot_dir = get_package_share_directory(
        'autopartol_robot'
    )

    # 2. 获取 YAML 配置文件路径
    patrol_config_path = os.path.join(
        autopartol_robot_dir,
        'config',
        'partol_config.yaml'
    )

    # 3. 启动巡逻节点
    action_node_patrol = launch_ros.actions.Node(
        package='autopartol_robot',
        executable='patrol_node',
        parameters=[patrol_config_path],
        output='screen'
    )

    # 4. 启动语音播报节点
    action_node_speaker = launch_ros.actions.Node(
        package='autopartol_robot',
        executable='speaker',
        output='screen'
    )

    # 5. 返回启动动作列表
    return launch.LaunchDescription([
        action_node_patrol,
        action_node_speaker,
    ])