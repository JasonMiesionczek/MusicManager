import React from 'react';
import { Platform } from 'react-native';
import { createStackNavigator, createBottomTabNavigator } from 'react-navigation';

import TabBarIcon from '../components/TabBarIcon';
import HomeScreen from '../screens/HomeScreen';
import LinksScreen from '../screens/LinksScreen';
import SettingsScreen from '../screens/SettingsScreen';
import AlbumsScreen from '../screens/AlbumsScreen';
import SongScreen from '../screens/SongScreen';
import PlaylistScreen from '../screens/PlaylistScreen';
import QueueScreen from '../screens/QueueScreen';

const LibraryStack = createStackNavigator({
  Home: HomeScreen,
  Albums: { screen: AlbumsScreen },
  Songs: { screen: SongScreen }
});

LibraryStack.navigationOptions = {
  tabBarLabel: 'Library',
  tabBarIcon: ({ focused }) => (
    <TabBarIcon
      focused={focused}
      name={
        Platform.OS === 'ios'
          ? `ios-list${focused ? '' : '-outline'}`
          : 'md-list'
      }
    />
  ),
};

const PlaylistStack = createStackNavigator({
  Playlists: PlaylistScreen
});

PlaylistStack.navigationOptions = {
  tabBarLabel: 'Playlists',
  tabBarIcon: ({ focused }) => (
    <TabBarIcon
      focused={focused}
      name={Platform.OS === 'ios' ? 'ios-play-circle' : 'md-play-circle'}
    />
  ),
};

const QueueStack = createStackNavigator({
  Queue: QueueScreen,
});

QueueStack.navigationOptions = {
  tabBarLabel: 'Queue',
  tabBarIcon: ({ focused }) => (
    <TabBarIcon
      focused={focused}
      name={Platform.OS === 'ios' ? 'ios-speedometer' : 'md-speedometer'}
    />
  ),
};

const SettingsStack = createStackNavigator({
  Settings: SettingsScreen,
});

SettingsStack.navigationOptions = {
  tabBarLabel: 'Settings',
  tabBarIcon: ({ focused }) => (
    <TabBarIcon
      focused={focused}
      name={Platform.OS === 'ios' ? 'ios-options' : 'md-options'}
    />
  ),
};

export default createBottomTabNavigator({
  LibraryStack,
  PlaylistStack,
  QueueStack,
  SettingsStack,
});
