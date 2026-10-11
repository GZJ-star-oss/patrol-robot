import os
import launch
import launch_ros
import launch_ros.parameter_descriptions
from ament_index_python.packages import get_package_share_directory
def generate_launch_description():
    # 获取默认路径
    urdf_tutorial_path = get_package_share_directory('fishbot_description')
    default_xacro_path = os.path.join(urdf_tutorial_path,'urdf','fishbot','fishbot.urdf.xacro')
   # default_rviz_config_path = urdf_tutorial_path + '/config/display_robot_model.rviz'
    default_gazebo_world_path = os.path.join(urdf_tutorial_path,'world','custrom_room.world')
    print("实际加载地图路径：", default_gazebo_world_path)

    # 为 launch file 声明一个参数
    action_declare_arg_mode_path = launch.actions.DeclareLaunchArgument(
        name = 'model',
        default_value = str(default_xacro_path),
        description='URDF 的绝对路径')
    # 获取文件内容生成新的参数
    robot_description = launch_ros.parameter_descriptions.ParameterValue(
        launch.substitutions.Command(['xacro ', launch.substitutions.LaunchConfiguration('model')]),
        value_type=str)
    # 状态发布节点
    robot_state_publisher_node = launch_ros.actions.Node(
        package='robot_state_publisher',
        executable='robot_state_publisher',
        parameters=[{'robot_description': robot_description}]
    )
    action_launch_gazebo =launch.actions.IncludeLaunchDescription(
        launch.launch_description_sources.PythonLaunchDescriptionSource(
            [get_package_share_directory('gazebo_ros'),'/launch','/gazebo.launch.py']),
        launch_arguments=[('world',default_gazebo_world_path),('verbose','true')],       
    )
    # 加载激活ros2_control
    action_load_joint_state_controller=launch.actions.ExecuteProcess(
        cmd=['ros2', 'control', 'load_controller', '--set-state', 'active', 'fishbot_joint_state_broadcaster'],
        output='screen')
    action_load_effort_controller=launch.actions.ExecuteProcess(
            cmd=['ros2', 'control', 'load_controller', '--set-state', 'active', 'fishbot_effort_controller'],
            output='screen')
    action_diff_driv_controller=launch.actions.ExecuteProcess(
                cmd=['ros2', 'control', 'load_controller', '--set-state', 'active', 'fishbot_diff_drive_controller'],
                output='screen')
    

    action_spawn_entity = launch_ros.actions.Node(
       package='gazebo_ros',
       executable= 'spawn_entity.py',
    arguments=['-topic','/robot_description','-entity','fishbot'])

    # )
    # 关节状态发布节点
    # joint_state_publisher_node = launch_ros.actions.Node(
    #     package='joint_state_publisher',
    #     executable='joint_state_publisher',
    # )
    # # # RViz 节点
    # rviz_node = launch_ros.actions.Node(
    #     package='rviz2',
    #     executable='rviz2',
    #     arguments=['-d', default_rviz_config_path]
    # )

    return launch.LaunchDescription([
        action_declare_arg_mode_path,
        #joint_state_publisher_node,
        robot_state_publisher_node,
        action_launch_gazebo,
        action_spawn_entity,
       # rviz_node
    #    注册一个监听器
      launch.actions.RegisterEventHandler(
            event_handler=launch.event_handlers.OnProcessExit(
                target_action=action_spawn_entity,
                on_exit=[action_load_joint_state_controller])),
      launch.actions.RegisterEventHandler(
         event_handler=launch.event_handlers.OnProcessExit(
            target_action=action_spawn_entity,
         on_exit=[action_diff_driv_controller]))


            ])
    
