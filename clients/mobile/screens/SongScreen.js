import React from 'react';
import {
    ImageBackground,
    Platform,
    ScrollView,
    StyleSheet,
    FlatList,
    Text,
    TouchableOpacity,
    View,
} from 'react-native';

export default class SongScreen extends React.Component {
    static navigationOptions = { header: null };
    constructor(props) {
        super(props);
        this.state = {
            songs: [],
            album: {},
            artist: {}
        }
    }

    async componentDidMount() {
        const albumId = this.props.navigation.state.params.albumId;
        const response = await fetch(`http://musicmanager.hopto.org:8000/api/library/songs/${albumId}`);
        const data = await response.json();
        this.navigationOptions = { header: data.album.name };
        this.setState({ songs: data.songs, album: data.album, artist: data.artist });
    }

    render() {
        return (
            <View style={styles.contentContainer}>
                <Text style={styles.artistName}>
                    {this.state.artist.name} - {this.state.album.name}
                </Text>
                <Text style={styles.albumCount}>
                    {this.state.songs.length} Songs
                </Text>
                <FlatList
                    data={this.state.songs}
                    renderItem={({ item }) => <TouchableOpacity onPress={() => { }}><ImageBackground style={{ width: 425, height: 100 }} source={{ uri: 'http://musicmanager.hopto.org:90/images/' + item.filename + '.png' }} ><Text style={styles.songName}>{item.name}</Text></ImageBackground></TouchableOpacity>}
                    keyExtractor={(item, index) => item.id + ""} />
            </View>
        )
    }
}

const styles = StyleSheet.create({
    container: {
        flex: 1,
        backgroundColor: '#fff',
    },
    albumCount: {
        paddingLeft: 25,
        color: '#ddd',
        fontSize: 20,
        paddingBottom: 20,
    },
    artistName: {
        color: '#fff',
        fontSize: 25,
        paddingTop: 10,
        paddingLeft: 25,
        height: 50,
        textShadowColor: '#000',
        textShadowOffset: { width: 2, height: 2 },
        textShadowRadius: 10
    },
    albumName: {
        color: '#fff',
        fontSize: 24,
        paddingTop: 125,
        paddingLeft: 25,
        textShadowColor: '#000',
        textShadowOffset: { width: 2, height: 2 },
        textShadowRadius: 10
    },
    songName: {
        color: '#fff',
        fontSize: 24,
        paddingTop: 25,
        paddingLeft: 25,
        textShadowColor: '#000',
        textShadowOffset: { width: 2, height: 2 },
        textShadowRadius: 10
    },
    developmentModeText: {
        marginBottom: 20,
        color: 'rgba(0,0,0,0.4)',
        fontSize: 14,
        lineHeight: 19,
        textAlign: 'center',
    },
    contentContainer: {
        paddingTop: 30,
        backgroundColor: '#222',
        paddingBottom: 94
    },
    welcomeContainer: {
        alignItems: 'center',
        marginTop: 10,
        marginBottom: 20,
    },
    welcomeImage: {
        width: 100,
        height: 80,
        resizeMode: 'contain',
        marginTop: 3,
        marginLeft: -10,
    },
    getStartedContainer: {
        alignItems: 'flex-start',
        marginHorizontal: 0,
    },
    homeScreenFilename: {
        marginVertical: 7,
    },
    codeHighlightText: {
        color: 'rgba(96,100,109, 0.8)',
    },
    codeHighlightContainer: {
        backgroundColor: 'rgba(0,0,0,0.05)',
        borderRadius: 3,
        paddingHorizontal: 4,
    },
    getStartedText: {
        fontSize: 17,
        color: 'rgba(96,100,109, 1)',
        lineHeight: 24,
        textAlign: 'center',
    },
    tabBarInfoContainer: {
        position: 'absolute',
        bottom: 0,
        left: 0,
        right: 0,
        ...Platform.select({
            ios: {
                shadowColor: 'black',
                shadowOffset: { height: -3 },
                shadowOpacity: 0.1,
                shadowRadius: 3,
            },
            android: {
                elevation: 20,
            },
        }),
        alignItems: 'center',
        backgroundColor: '#fbfbfb',
        paddingVertical: 20,
    },
    tabBarInfoText: {
        fontSize: 17,
        color: 'rgba(96,100,109, 1)',
        textAlign: 'center',
    },
    navigationFilename: {
        marginTop: 5,
    },
    helpContainer: {
        marginTop: 15,
        alignItems: 'center',
    },
    helpLink: {
        paddingVertical: 15,
    },
    helpLinkText: {
        fontSize: 14,
        color: '#2e78b7',
    },
});
