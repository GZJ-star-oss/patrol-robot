// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from autopatol_interfaces:srv/SpeechText.idl
// generated code does not contain a copyright notice

#ifndef AUTOPATOL_INTERFACES__SRV__DETAIL__SPEECH_TEXT__BUILDER_HPP_
#define AUTOPATOL_INTERFACES__SRV__DETAIL__SPEECH_TEXT__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "autopatol_interfaces/srv/detail/speech_text__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace autopatol_interfaces
{

namespace srv
{

namespace builder
{

class Init_SpeechText_Request_text
{
public:
  Init_SpeechText_Request_text()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::autopatol_interfaces::srv::SpeechText_Request text(::autopatol_interfaces::srv::SpeechText_Request::_text_type arg)
  {
    msg_.text = std::move(arg);
    return std::move(msg_);
  }

private:
  ::autopatol_interfaces::srv::SpeechText_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::autopatol_interfaces::srv::SpeechText_Request>()
{
  return autopatol_interfaces::srv::builder::Init_SpeechText_Request_text();
}

}  // namespace autopatol_interfaces


namespace autopatol_interfaces
{

namespace srv
{

namespace builder
{

class Init_SpeechText_Response_result
{
public:
  Init_SpeechText_Response_result()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::autopatol_interfaces::srv::SpeechText_Response result(::autopatol_interfaces::srv::SpeechText_Response::_result_type arg)
  {
    msg_.result = std::move(arg);
    return std::move(msg_);
  }

private:
  ::autopatol_interfaces::srv::SpeechText_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::autopatol_interfaces::srv::SpeechText_Response>()
{
  return autopatol_interfaces::srv::builder::Init_SpeechText_Response_result();
}

}  // namespace autopatol_interfaces

#endif  // AUTOPATOL_INTERFACES__SRV__DETAIL__SPEECH_TEXT__BUILDER_HPP_
